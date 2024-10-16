//! This module is a partial implementation of the CSS Grid Level 1 specification
//! <https://www.w3.org/TR/css-grid-1>
use core::borrow::Borrow;

use crate::geometry::{AbsoluteAxis, AbstractAxis, InBothAbsAxis};
use crate::geometry::{Line, Point, Rect, Size};
use crate::style::{AlignItems, AlignSelf, AvailableSpace, Overflow, Position};
use crate::tree::{Layout, LayoutInput, LayoutOutput, LayoutPartialTreeExt, NodeId, RunMode, SizingMode};
use crate::util::debug::debug_log;
use crate::util::sys::{f32_max, GridTrackVec, Vec};
use crate::util::MaybeMath;
use crate::util::{MaybeResolve, ResolveOrZero};
use crate::{
    style_helpers::*, AlignContent, BoxGenerationMode, BoxSizing, CoreStyle, Direction, GridContainerStyle,
    GridItemStyle, JustifyContent, LayoutGridContainer,
};
use alignment::{align_and_position_item, align_tracks};
use explicit_grid::{compute_explicit_grid_size_in_axis, initialize_grid_tracks};
use implicit_grid::compute_grid_size_estimate;
use placement::place_grid_items;
use track_sizing::{
    determine_if_item_crosses_flexible_or_intrinsic_tracks, resolve_item_track_indexes, track_sizing_algorithm,
};
use types::{CellOccupancyMatrix, GridTrack};

pub(crate) use types::{GridCoordinate, GridLine, OriginZeroLine};

mod alignment;
mod explicit_grid;
mod implicit_grid;
mod placement;
mod track_sizing;
mod types;
mod util;

/// Grid layout algorithm
/// This consists of a few phases:
///   - Resolving the explicit grid
///   - Placing items (which also resolves the implicit grid)
///   - Track (row/column) sizing
///   - Alignment & Final item placement
pub fn compute_grid_layout(tree: &mut impl LayoutGridContainer, node: NodeId, inputs: LayoutInput) -> LayoutOutput {
    let LayoutInput { direction, known_dimensions, parent_size, available_space, run_mode, .. } = inputs;

    let style = tree.get_grid_container_style(node);

    // 1. Compute "available grid space"
    // https://www.w3.org/TR/css-grid-1/#available-grid-space
    let aspect_ratio = style.aspect_ratio();
    let padding = style.padding().resolve_or_zero(parent_size.width);
    let border = style.border().resolve_or_zero(parent_size.width);
    let padding_border = padding + border;
    let padding_border_size = padding_border.sum_axes();
    let box_sizing_adjustment =
        if style.box_sizing() == BoxSizing::ContentBox { padding_border_size } else { Size::ZERO };

    let min_size = style
        .min_size()
        .maybe_resolve(parent_size)
        .maybe_apply_aspect_ratio(aspect_ratio)
        .maybe_add(box_sizing_adjustment);
    let max_size = style
        .max_size()
        .maybe_resolve(parent_size)
        .maybe_apply_aspect_ratio(aspect_ratio)
        .maybe_add(box_sizing_adjustment);
    let preferred_size = if inputs.sizing_mode == SizingMode::InherentSize {
        style
            .size()
            .maybe_resolve(parent_size)
            .maybe_apply_aspect_ratio(style.aspect_ratio())
            .maybe_add(box_sizing_adjustment)
    } else {
        Size::NONE
    };

    // Scrollbar gutters are reserved when the `overflow` property is set to `Overflow::Scroll`.
    // However, the axis are switched (transposed) because a node that scrolls vertically needs
    // *horizontal* space to be reserved for a scrollbar
    let scrollbar_gutter = style.overflow().transpose().map(|overflow| match overflow {
        Overflow::Scroll => style.scrollbar_width(),
        _ => 0.0,
    });

    let mut content_box_inset = padding_border;
    match direction {
        Direction::Ltr => content_box_inset.right += scrollbar_gutter.x,
        Direction::Rtl => content_box_inset.left += scrollbar_gutter.x,
    };
    content_box_inset.bottom += scrollbar_gutter.y;

    let align_content = style.align_content().unwrap_or(AlignContent::Stretch);
    let mut justify_content = style.justify_content().unwrap_or(JustifyContent::Stretch);
    let align_items = style.align_items();
    let mut justify_items = style.justify_items();

    match direction {
        Direction::Rtl => {
            justify_content.flip();
            justify_items.as_mut().map(|x| x.flip());
        }
        _ => (),
    }

    // Note: we avoid accessing the grid rows/columns methods more than once as this can
    // cause an expensive-ish computation
    let grid_template_columms = style.grid_template_columns();
    let grid_template_rows = style.grid_template_rows();
    let grid_auto_columms = style.grid_auto_columns();
    let grid_auto_rows = style.grid_auto_rows();

    let constrained_available_space = known_dimensions
        .or(preferred_size)
        .map(|size| size.map(AvailableSpace::Definite))
        .unwrap_or(available_space)
        .maybe_clamp(min_size, max_size)
        .maybe_max(padding_border_size);

    let available_grid_space = Size {
        width: constrained_available_space
            .width
            .map_definite_value(|space| space - content_box_inset.horizontal_axis_sum()),
        height: constrained_available_space
            .height
            .map_definite_value(|space| space - content_box_inset.vertical_axis_sum()),
    };

    let outer_node_size =
        known_dimensions.or(preferred_size).maybe_clamp(min_size, max_size).maybe_max(padding_border_size);
    let mut inner_node_size = Size {
        width: outer_node_size.width.map(|space| space - content_box_inset.horizontal_axis_sum()),
        height: outer_node_size.height.map(|space| space - content_box_inset.vertical_axis_sum()),
    };

    debug_log!("parent_size", dbg:parent_size);
    debug_log!("outer_node_size", dbg:outer_node_size);
    debug_log!("inner_node_size", dbg:inner_node_size);

    if let (RunMode::ComputeSize, Some(width), Some(height)) = (run_mode, outer_node_size.width, outer_node_size.height)
    {
        return LayoutOutput::from_outer_size(Size { width, height });
    }

    let get_child_styles_iter =
        |node| tree.child_ids(node).map(|child_node: NodeId| tree.get_grid_child_style(child_node));
    let child_styles_iter = get_child_styles_iter(node);

    // 2. Resolve the explicit grid

    // This is very similar to the inner_node_size except if the inner_node_size is not definite but the node
    // has a min- or max- size style then that will be used in it's place.
    let auto_fit_container_size = outer_node_size
        .or(max_size)
        .or(min_size)
        .maybe_clamp(min_size, max_size)
        .maybe_max(padding_border_size)
        .maybe_sub(content_box_inset.sum_axes());

    // Exactly compute the number of rows and columns in the explicit grid.
    let explicit_col_count = compute_explicit_grid_size_in_axis(
        &style,
        grid_template_columms.borrow(),
        auto_fit_container_size,
        AbsoluteAxis::Horizontal,
    );
    let explicit_row_count = compute_explicit_grid_size_in_axis(
        &style,
        grid_template_rows.borrow(),
        auto_fit_container_size,
        AbsoluteAxis::Vertical,
    );

    // 3. Implicit Grid: Estimate Track Counts
    // Estimate the number of rows and columns in the implicit grid (= the entire grid)
    // This is necessary as part of placement. Doing it early here is a perf optimisation to reduce allocations.
    let (est_col_counts, est_row_counts) =
        compute_grid_size_estimate(explicit_col_count, explicit_row_count, child_styles_iter);

    // 4. Grid Item Placement
    // Match items (children) to a definite grid position (row start/end and column start/end position)
    let mut items = Vec::with_capacity(tree.child_count(node));
    let mut cell_occupancy_matrix = CellOccupancyMatrix::with_track_counts(est_col_counts, est_row_counts);
    let in_flow_children_iter = || {
        tree.child_ids(node)
            .enumerate()
            .map(|(index, child_node)| (index, child_node, tree.get_grid_child_style(child_node)))
            .filter(|(_, _, style)| {
                style.box_generation_mode() != BoxGenerationMode::None && style.position() != Position::Absolute
            })
    };
    place_grid_items(
        &mut cell_occupancy_matrix,
        &mut items,
        in_flow_children_iter,
        style.grid_auto_flow(),
        align_items.unwrap_or(AlignItems::Stretch),
        justify_items.unwrap_or(AlignItems::Stretch),
    );

    // Extract track counts from previous step (auto-placement can expand the number of tracks)
    let final_col_counts = *cell_occupancy_matrix.track_counts(AbsoluteAxis::Horizontal);
    let final_row_counts = *cell_occupancy_matrix.track_counts(AbsoluteAxis::Vertical);

    // 5. Initialize Tracks
    // Initialize (explicit and implicit) grid tracks (and gutters)
    // This resolves the min and max track sizing functions for all tracks and gutters
    let mut columns = GridTrackVec::new();
    let mut rows = GridTrackVec::new();
    initialize_grid_tracks(
        &mut columns,
        final_col_counts,
        grid_template_columms.borrow(),
        grid_auto_columms.borrow(),
        style.gap().width,
        |column_index| cell_occupancy_matrix.column_is_occupied(column_index),
    );
    initialize_grid_tracks(
        &mut rows,
        final_row_counts,
        grid_template_rows.borrow(),
        grid_auto_rows.borrow(),
        style.gap().height,
        |row_index| cell_occupancy_matrix.row_is_occupied(row_index),
    );

    drop(grid_template_rows);
    drop(grid_template_columms);
    drop(grid_auto_rows);
    drop(grid_auto_columms);
    drop(style);

    // 6. Track Sizing

    // Convert grid placements in origin-zero coordinates to indexes into the GridTrack (rows and columns) vectors
    // This computation is relatively trivial, but it requires the final number of negative (implicit) tracks in
    // each axis, and doing it up-front here means we don't have to keep repeating that calculation
    resolve_item_track_indexes(&mut items, final_col_counts, final_row_counts);

    // For each item, and in each axis, determine whether the item crosses any flexible (fr) tracks
    // Record this as a boolean (per-axis) on each item for later use in the track-sizing algorithm
    determine_if_item_crosses_flexible_or_intrinsic_tracks(&mut items, &columns, &rows);

    // Determine if the grid has any baseline aligned items
    let has_baseline_aligned_item = items.iter().any(|item| item.align_self == AlignSelf::Baseline);

    // Run track sizing algorithm for Inline axis
    track_sizing_algorithm(
        tree,
        AbstractAxis::Inline,
        min_size.get(AbstractAxis::Inline),
        max_size.get(AbstractAxis::Inline),
        align_content,
        available_grid_space,
        inner_node_size,
        &mut columns,
        &mut rows,
        &mut items,
        |track: &GridTrack, parent_size: Option<f32>| track.max_track_sizing_function.definite_value(parent_size),
        has_baseline_aligned_item,
    );
    let initial_column_sum = columns.iter().map(|track| track.base_size).sum::<f32>();
    inner_node_size.width = inner_node_size.width.or_else(|| initial_column_sum.into());

    items.iter_mut().for_each(|item| item.available_space_cache = None);

    // Run track sizing algorithm for Block axis
    track_sizing_algorithm(
        tree,
        AbstractAxis::Block,
        min_size.get(AbstractAxis::Block),
        max_size.get(AbstractAxis::Block),
        justify_content,
        available_grid_space,
        inner_node_size,
        &mut rows,
        &mut columns,
        &mut items,
        |track: &GridTrack, _| Some(track.base_size),
        false, // TODO: Support baseline alignment in the vertical axis
    );
    let initial_row_sum = rows.iter().map(|track| track.base_size).sum::<f32>();
    inner_node_size.height = inner_node_size.height.or_else(|| initial_row_sum.into());

    debug_log!("initial_column_sum", dbg:initial_column_sum);
    debug_log!(dbg: columns.iter().map(|track| track.base_size).collect::<Vec<_>>());
    debug_log!("initial_row_sum", dbg:initial_row_sum);
    debug_log!(dbg: rows.iter().map(|track| track.base_size).collect::<Vec<_>>());

    // 6. Compute container size
    let resolved_style_size = known_dimensions.or(preferred_size);
    let container_border_box = Size {
        width: resolved_style_size
            .get(AbstractAxis::Inline)
            .unwrap_or_else(|| initial_column_sum + content_box_inset.horizontal_axis_sum())
            .maybe_clamp(min_size.width, max_size.width)
            .max(padding_border_size.width),
        height: resolved_style_size
            .get(AbstractAxis::Block)
            .unwrap_or_else(|| initial_row_sum + content_box_inset.vertical_axis_sum())
            .maybe_clamp(min_size.height, max_size.height)
            .max(padding_border_size.height),
    };
    let container_content_box = Size {
        width: f32_max(0.0, container_border_box.width - content_box_inset.horizontal_axis_sum()),
        height: f32_max(0.0, container_border_box.height - content_box_inset.vertical_axis_sum()),
    };

    // If only the container's size has been requested
    if run_mode == RunMode::ComputeSize {
        return LayoutOutput::from_outer_size(container_border_box);
    }

    // 7. Resolve percentage track base sizes
    // In the case of an indefinitely sized container these resolve to zero during the "Initialise Tracks" step
    // and therefore need to be re-resolved here based on the content-sized content box of the container
    if !available_grid_space.width.is_definite() {
        for column in &mut columns {
            let min: Option<f32> =
                column.min_track_sizing_function.resolved_percentage_size(container_content_box.width);
            let max: Option<f32> =
                column.max_track_sizing_function.resolved_percentage_size(container_content_box.width);
            column.base_size = column.base_size.maybe_clamp(min, max);
        }
    }
    if !available_grid_space.height.is_definite() {
        for row in &mut rows {
            let min: Option<f32> = row.min_track_sizing_function.resolved_percentage_size(container_content_box.height);
            let max: Option<f32> = row.max_track_sizing_function.resolved_percentage_size(container_content_box.height);
            row.base_size = row.base_size.maybe_clamp(min, max);
        }
    }

    // Column sizing must be re-run (once) if:
    //   - The grid container's width was initially indefinite and there are any columns with percentage track sizing functions
    //   - Any grid item crossing an intrinsically sized track's min content contribution width has changed
    // TODO: Only rerun sizing for tracks that actually require it rather than for all tracks if any need it.
    let mut rerun_column_sizing;

    let has_percentage_column = columns.iter().any(|track| track.uses_percentage());
    let parent_width_indefinite = !available_space.width.is_definite();
    rerun_column_sizing = parent_width_indefinite && has_percentage_column;

    if !rerun_column_sizing {
        let min_content_contribution_changed = items
            .iter_mut()
            .filter(|item| item.crosses_intrinsic_column)
            .map(|item| {
                let available_space = item.available_space(
                    AbstractAxis::Inline,
                    &rows,
                    inner_node_size.height,
                    |track: &GridTrack, _| Some(track.base_size),
                );
                let new_min_content_contribution =
                    item.min_content_contribution(AbstractAxis::Inline, tree, available_space, inner_node_size);

                let has_changed = Some(new_min_content_contribution) != item.min_content_contribution_cache.width;

                item.available_space_cache = Some(available_space);
                item.min_content_contribution_cache.width = Some(new_min_content_contribution);
                item.max_content_contribution_cache.width = None;
                item.minimum_contribution_cache.width = None;

                has_changed
            })
            .any(|has_changed| has_changed);
        rerun_column_sizing = min_content_contribution_changed;
    } else {
        // Clear intrisic width caches
        items.iter_mut().for_each(|item| {
            item.available_space_cache = None;
            item.min_content_contribution_cache.width = None;
            item.max_content_contribution_cache.width = None;
            item.minimum_contribution_cache.width = None;
        });
    }

    if rerun_column_sizing {
        // Re-run track sizing algorithm for Inline axis
        track_sizing_algorithm(
            tree,
            AbstractAxis::Inline,
            min_size.get(AbstractAxis::Inline),
            max_size.get(AbstractAxis::Inline),
            align_content,
            available_grid_space,
            inner_node_size,
            &mut columns,
            &mut rows,
            &mut items,
            |track: &GridTrack, _| Some(track.base_size),
            has_baseline_aligned_item,
        );

        // Row sizing must be re-run (once) if:
        //   - The grid container's height was initially indefinite and there are any rows with percentage track sizing functions
        //   - Any grid item crossing an intrinsically sized track's min content contribution height has changed
        // TODO: Only rerun sizing for tracks that actually require it rather than for all tracks if any need it.
        let mut rerun_row_sizing;

        let has_percentage_row = rows.iter().any(|track| track.uses_percentage());
        let parent_height_indefinite = !available_space.height.is_definite();
        rerun_row_sizing = parent_height_indefinite && has_percentage_row;

        if !rerun_row_sizing {
            let min_content_contribution_changed = items
                .iter_mut()
                .filter(|item| item.crosses_intrinsic_column)
                .map(|item| {
                    let available_space = item.available_space(
                        AbstractAxis::Block,
                        &columns,
                        inner_node_size.width,
                        |track: &GridTrack, _| Some(track.base_size),
                    );
                    let new_min_content_contribution =
                        item.min_content_contribution(AbstractAxis::Block, tree, available_space, inner_node_size);

                    let has_changed = Some(new_min_content_contribution) != item.min_content_contribution_cache.height;

                    item.available_space_cache = Some(available_space);
                    item.min_content_contribution_cache.height = Some(new_min_content_contribution);
                    item.max_content_contribution_cache.height = None;
                    item.minimum_contribution_cache.height = None;

                    has_changed
                })
                .any(|has_changed| has_changed);
            rerun_row_sizing = min_content_contribution_changed;
        } else {
            items.iter_mut().for_each(|item| {
                // Clear intrisic height caches
                item.available_space_cache = None;
                item.min_content_contribution_cache.height = None;
                item.max_content_contribution_cache.height = None;
                item.minimum_contribution_cache.height = None;
            });
        }

        if rerun_row_sizing {
            // Re-run track sizing algorithm for Block axis
            track_sizing_algorithm(
                tree,
                AbstractAxis::Block,
                min_size.get(AbstractAxis::Block),
                max_size.get(AbstractAxis::Block),
                justify_content,
                available_grid_space,
                inner_node_size,
                &mut rows,
                &mut columns,
                &mut items,
                |track: &GridTrack, _| Some(track.base_size),
                false, // TODO: Support baseline alignment in the vertical axis
            );
        }
    }

    // 8. Track Alignment

    // Align columns
    align_tracks(
        container_content_box.get(AbstractAxis::Inline),
        Line { start: padding.left, end: padding.right },
        Line { start: border.left, end: border.right },
        &mut columns,
        justify_content,
    );
    // Align rows
    align_tracks(
        container_content_box.get(AbstractAxis::Block),
        Line { start: padding.top, end: padding.bottom },
        Line { start: border.top, end: border.bottom },
        &mut rows,
        align_content,
    );

    // 9. Size, Align, and Position Grid Items

    #[cfg_attr(not(feature = "content_size"), allow(unused_mut))]
    let mut item_content_size_contribution = Size::ZERO;

    // Sort items back into original order to allow them to be matched up with styles
    items.sort_by_key(|item| item.source_order);

    let container_alignment_styles = InBothAbsAxis { horizontal: justify_items, vertical: align_items };

    // Position in-flow children (stored in items vector)
    for (index, item) in items.iter_mut().enumerate() {
        let (left, right) = match item.direction {
            Direction::Ltr => (
                columns[item.column_indexes.start as usize + 1].offset,
                columns[item.column_indexes.end as usize].offset,
            ),
            Direction::Rtl => (
                container_border_box.width - columns[item.column_indexes.end as usize].offset,
                container_border_box.width - columns[item.column_indexes.start as usize + 1].offset,
            ),
        };

        let grid_area = Rect {
            top: rows[item.row_indexes.start as usize + 1].offset,
            bottom: rows[item.row_indexes.end as usize].offset,
            left,
            right,
        };
        #[cfg_attr(not(feature = "content_size"), allow(unused_variables))]
        let (content_size_contribution, y_position, height) = align_and_position_item(
            tree,
            item.node,
            index as u32,
            grid_area,
            container_alignment_styles,
            item.baseline_shim,
        );
        item.y_position = y_position;
        item.height = height;

        #[cfg(feature = "content_size")]
        {
            item_content_size_contribution = item_content_size_contribution.f32_max(content_size_contribution);
        }
    }

    // Position hidden and absolutely positioned children
    let mut order = items.len() as u32;
    (0..tree.child_count(node)).for_each(|index| {
        let child = tree.get_child_id(node, index);
        let child_style = tree.get_grid_child_style(child);

        // Position hidden child
        if child_style.box_generation_mode() == BoxGenerationMode::None {
            let direction = child_style.direction();
            drop(child_style);
            tree.set_unrounded_layout(child, &Layout::with_order(order));
            tree.perform_child_layout(
                child,
                Size::NONE,
                Size::NONE,
                Size::MAX_CONTENT,
                SizingMode::InherentSize,
                direction,
                Line::FALSE,
            );
            order += 1;
            return;
        }

        // Position absolutely positioned child
        if child_style.position() == Position::Absolute {
            // Convert grid-col-{start/end} into Option's of indexes into the columns vector
            // The Option is None if the style property is Auto and an unresolvable Span
            let maybe_col_indexes = child_style
                .grid_column()
                .into_origin_zero(final_col_counts.explicit)
                .resolve_absolutely_positioned_grid_tracks()
                .map(|maybe_grid_line| {
                    maybe_grid_line.map(|line: OriginZeroLine| line.into_track_vec_index(final_col_counts))
                });
            // Convert grid-row-{start/end} into Option's of indexes into the row vector
            // The Option is None if the style property is Auto and an unresolvable Span
            let maybe_row_indexes = child_style
                .grid_row()
                .into_origin_zero(final_row_counts.explicit)
                .resolve_absolutely_positioned_grid_tracks()
                .map(|maybe_grid_line| {
                    maybe_grid_line.map(|line: OriginZeroLine| line.into_track_vec_index(final_row_counts))
                });

            let grid_area = Rect {
                top: maybe_row_indexes.start.map(|index| rows[index].offset).unwrap_or(border.top),
                bottom: maybe_row_indexes
                    .end
                    .map(|index| rows[index].offset)
                    .unwrap_or(container_border_box.height - border.bottom - scrollbar_gutter.y),
                left: maybe_col_indexes.start.map(|index| columns[index].offset).unwrap_or(border.left),
                right: maybe_col_indexes
                    .end
                    .map(|index| columns[index].offset)
                    .unwrap_or(container_border_box.width - border.right - scrollbar_gutter.x),
            };
            drop(child_style);

            // TODO: Baseline alignment support for absolutely positioned items (should check if is actuallty specified)
            #[cfg_attr(not(feature = "content_size"), allow(unused_variables))]
            let (content_size_contribution, _, _) =
                align_and_position_item(tree, child, order, grid_area, container_alignment_styles, 0.0);
            #[cfg(feature = "content_size")]
            {
                item_content_size_contribution = item_content_size_contribution.f32_max(content_size_contribution);
            }

            order += 1;
        }
    });

    // If there are not items then return just the container size (no baseline)
    if items.is_empty() {
        return LayoutOutput::from_outer_size(container_border_box);
    }

    // Determine the grid container baseline(s) (currently we only compute the first baseline)
    let grid_container_baseline: f32 = {
        // Sort items by row start position so that we can iterate items in groups which are in the same row
        items.sort_by_key(|item| item.row_indexes.start);

        // Get the row index of the first row containing items
        let first_row = items[0].row_indexes.start;

        // Create a slice of all of the items start in this row (taking advantage of the fact that we have just sorted the array)
        let first_row_items = &items[0..].split(|item| item.row_indexes.start != first_row).next().unwrap();

        // Check if any items in *this row* are baseline aligned
        let row_has_baseline_item = first_row_items.iter().any(|item| item.align_self == AlignSelf::Baseline);

        let item = if row_has_baseline_item {
            first_row_items.iter().find(|item| item.align_self == AlignSelf::Baseline).unwrap()
        } else {
            &first_row_items[0]
        };

        item.y_position + item.baseline.unwrap_or(item.height)
    };

    LayoutOutput::from_sizes_and_baselines(
        container_border_box,
        item_content_size_contribution,
        Point { x: None, y: Some(grid_container_baseline) },
    )
}
