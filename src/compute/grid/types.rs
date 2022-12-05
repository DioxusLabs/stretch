use super::placement::{CellOccupancyMatrix, TrackCounts};
use crate::compute::compute_node_layout;
use crate::geometry::{Line, Size};
use crate::layout::{AvailableSpace, AvailableSpaceCache, RunMode, SizingMode};
use crate::node::Node;
use crate::prelude::LayoutTree;
use crate::resolve::MaybeResolve;
use crate::style::{Dimension, MaxTrackSizingFunction, MinTrackSizingFunction, TrackSizingFunction};
use crate::sys::GridTrackVec;
use core::cmp::max;
use core::ops::Range;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AbsoluteAxis {
    Horizontal,
    Vertical,
}

impl AbsoluteAxis {
    #[inline]
    pub const fn other_axis(&self) -> Self {
        match *self {
            AbsoluteAxis::Horizontal => AbsoluteAxis::Vertical,
            AbsoluteAxis::Vertical => AbsoluteAxis::Horizontal,
        }
    }

    #[inline]
    pub fn into_column_row<T>(&self, primary: T, secondary: T) -> (T, T) {
        match *self {
            AbsoluteAxis::Horizontal => (primary, secondary),
            AbsoluteAxis::Vertical => (secondary, primary),
        }
    }

    #[inline]
    pub fn into_primary_secondary<T>(&self, row: T, column: T) -> (T, T) {
        match *self {
            AbsoluteAxis::Horizontal => (row, column),
            AbsoluteAxis::Vertical => (column, row),
        }
    }
}

impl<T> Size<T> {
    #[inline(always)]
    pub fn get_abs(self, axis: AbsoluteAxis) -> T {
        match axis {
            AbsoluteAxis::Horizontal => self.width,
            AbsoluteAxis::Vertical => self.height,
        }
    }

    #[inline(always)]
    pub fn get_abs_other(self, axis: AbsoluteAxis) -> T {
        match axis {
            AbsoluteAxis::Horizontal => self.height,
            AbsoluteAxis::Vertical => self.width,
        }
    }
}

/// The abstract axis in CSS Grid
#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum GridAxis {
    /// The axis in the inline dimension, i.e. the horizontal axis in horizontal writing modes and the vertical axis in vertical writing modes.
    Inline,
    /// The axis in the block dimension, i.e. the vertical axis in horizontal writing modes and the horizontal axis in vertical writing modes.
    Block,
}

impl GridAxis {
    pub fn other(&self) -> GridAxis {
        match *self {
            GridAxis::Inline => GridAxis::Block,
            GridAxis::Block => GridAxis::Inline,
        }
    }
}

/// Whether a GridTrack represents an actual track or a gutter.
#[derive(Copy, Clone, Debug, PartialEq)]
pub(in super::super) enum GridTrackKind {
    Uninit,
    Track,
    Gutter, // { name: Option<u16> },
}

/// Internal sizing information for a single grid track (row/column)
/// Gutters between tracks are sized similarly to actual tracks, so they
/// are also represented by this struct
#[derive(Debug, Clone)]
pub(in super::super) struct GridTrack {
    /// Whether the track is a full track, a gutter, or a placeholder that has not yet been initialised
    pub kind: GridTrackKind,

    /// The minimum track sizing function of the track
    pub min_track_sizing_function: MinTrackSizingFunction,

    /// The maximum track sizing function of the track
    pub max_track_sizing_function: MaxTrackSizingFunction,

    /// The distance of the start of the track from the start of the grid container
    pub offset: f32,

    /// The size (width/height as applicable) of the track
    pub base_size: f32,

    /// A temporary scratch value when sizing tracks
    /// Note: can be infinity
    pub growth_limit: f32,

    /// A temporary scratch value when "distributing space" to avoid clobbering planned increase variable
    pub item_incurred_increase: f32,
    /// A temporary scratch value when "distributing space" to avoid clobbering the main variable
    pub base_size_planned_increase: f32,
    /// A temporary scratch value when "distributing space" to avoid clobbering the main variable
    pub growth_limit_planned_increase: f32,
    /// A temporary scratch value when "distributing space"
    /// See: https://www.w3.org/TR/css3-grid-layout/#infinitely-growable
    pub infinitely_growable: bool,
}

impl GridTrack {
    pub fn new(
        min_track_sizing_function: MinTrackSizingFunction,
        max_track_sizing_function: MaxTrackSizingFunction,
    ) -> GridTrack {
        GridTrack {
            kind: GridTrackKind::Track,
            min_track_sizing_function,
            max_track_sizing_function,
            offset: 0.0,
            base_size: 0.0,
            growth_limit: 0.0,
            item_incurred_increase: 0.0,
            base_size_planned_increase: 0.0,
            growth_limit_planned_increase: 0.0,
            infinitely_growable: false,
        }
    }

    pub fn gutter(size: Dimension) -> GridTrack {
        GridTrack {
            kind: GridTrackKind::Gutter, // { name: None },
            min_track_sizing_function: MinTrackSizingFunction::Fixed(size),
            max_track_sizing_function: MaxTrackSizingFunction::Fixed(size),
            offset: 0.0,
            base_size: 0.0,
            growth_limit: 0.0,
            item_incurred_increase: 0.0,
            base_size_planned_increase: 0.0,
            growth_limit_planned_increase: 0.0,
            infinitely_growable: false,
        }
    }

    pub fn uninit() -> GridTrack {
        GridTrack {
            kind: GridTrackKind::Uninit,
            min_track_sizing_function: MinTrackSizingFunction::Auto,
            max_track_sizing_function: MaxTrackSizingFunction::Auto,
            offset: 0.0,
            base_size: 0.0,
            growth_limit: 0.0,
            item_incurred_increase: 0.0,
            base_size_planned_increase: 0.0,
            growth_limit_planned_increase: 0.0,
            infinitely_growable: false,
        }
    }

    #[inline]
    pub fn is_flexible(&self) -> bool {
        match self.max_track_sizing_function {
            MaxTrackSizingFunction::Flex(_) => true,
            _ => false,
        }
    }

    #[inline]
    pub fn flex_factor(&self) -> f32 {
        match self.max_track_sizing_function {
            MaxTrackSizingFunction::Flex(flex_factor) => flex_factor,
            _ => 0.0,
        }
    }
}

pub(super) trait GridAxisExt {
    fn flex_factor_sum(&self) -> f32;
    fn leftover_space(&self) -> f32;
}

#[derive(Debug, Clone)]
pub(super) struct GridAxisTracks {
    pub tracks: GridTrackVec<GridTrack>,
    pub origin: u16,
    pub track_counts: TrackCounts,
    // pub explicit_track_count: u16,
    // pub negative_implicit_track_count: u16,
    // pub positive_implicit_track_count: u16,
}

impl GridAxisTracks {
    #[inline]
    pub fn with_counts(counts: TrackCounts) -> GridAxisTracks {
        GridAxisTracks {
            tracks: GridTrackVec::with_capacity((counts.len() * 2) + 1),
            origin: counts.negative_implicit + 1,
            track_counts: counts,
        }
    }

    #[inline]
    pub fn len(&mut self) -> u16 {
        self.len() as u16
    }

    #[inline]
    pub fn push(&mut self, item: GridTrack) {
        self.push(item)
    }

    /// The lowest index initialised track
    pub fn min_track(&self) -> i16 {
        if self.track_counts.negative_implicit > 1 {
            -(self.track_counts.negative_implicit as i16)
        } else {
            1
        }
    }

    /// The highest index initialised track
    pub fn max_track(&self) -> i16 {
        (self.track_counts.explicit + self.track_counts.positive_implicit) as i16
    }

    /// Amount of space allocated for negative tracks
    pub fn negative_track_capacity(&self) -> u16 {
        self.origin / 2
    }

    /// Amount of space allocated for positive tracks
    pub fn positive_track_capacity(&self) -> u16 {
        (self.tracks.len() as u16 - self.origin) / 2 // Note: this relies on integer division rounding the odd number down
    }

    /// Get a track's index in self.tracks given its index as defined in CSS grid coordinates
    fn get_track_index_raw(&self, index: i16) -> i16 {
        use core::cmp::Ordering;

        // Compute the index of the track in the tracks vector based on its CSS grid index
        // taking into account:
        //   - Zero is not a valid index
        //   - CSS grid indexes are 1-based, but the tracks vector is 0-based
        //   - Gutters are also stored in the tracks vector
        //   - Tracks in the tracks vector may be offset due to negative tracks
        //   - The passed index may be negative, which should resolve backwards from the end of the explicit grid
        let computed_index: i16 = match index.cmp(&0) {
            Ordering::Equal => 0,
            Ordering::Less => {
                max(0, (self.origin + self.track_counts.explicit * 2) as i16 - (index.abs() as i16 * 2 - 1))
            }
            Ordering::Greater => (self.origin as i16) + (index * 2 - 1),
        };

        computed_index
    }

    /// Get a track's index in self.tracks given its index as defined in CSS grid coordinates
    /// With bounds checks
    fn get_initialized_track_index(&self, index: i16) -> Option<usize> {
        if index == 0 || index < self.min_track() || index > self.max_track() {
            return None;
        }
        let computed_index = self.get_track_index_raw(index);

        // Logic in above match block + correctly maintained values for the *_count variables
        // mean that the function should already have returned None when the index is out of range.
        debug_assert!(computed_index < 0, "index out of range (too small)");
        debug_assert!(computed_index as isize > self.tracks.len() as isize, "index out of range (too large)");

        Some(computed_index as usize)
    }

    /// Check if is initialised (either as an explicit or implicit track) given its index as defined in CSS grid coordinates
    pub fn has_explicit_track(&self, index: i16) -> bool {
        index > 0 && index <= self.track_counts.explicit as i16
    }

    /// Check if is initialised (as an explicit track) given its index as defined in CSS grid coordinates
    pub fn has_track(&self, index: i16) -> bool {
        self.get_initialized_track_index(index).is_some()
    }

    /// Retrieve a track by its index as defined in CSS grid coordinates
    pub fn get_track(&self, index: i16) -> Option<&GridTrack> {
        self.get_initialized_track_index(index).and_then(|index| self.tracks.get(index))
    }

    /// Retrieve a track by its index as defined in CSS grid coordinates
    pub fn get_track_mut(&mut self, index: i16) -> Option<&mut GridTrack> {
        self.get_initialized_track_index(index).and_then(|index| self.tracks.get_mut(index))
    }

    pub fn extend_implict_grid_to(
        &mut self,
        mut min_index: i16,
        mut max_index: i16,
        gap: Dimension,
        auto_tracks: Vec<TrackSizingFunction>,
    ) {
        if min_index == 0 || max_index == 0 {
            panic!("Track index cannot be zero");
        }
        if min_index > max_index {
            (min_index, max_index) = (max_index, min_index);
        }

        // Calculate required capacity
        let mut required_negative_capacity = 0;
        let mut new_negative_tracks = 0;
        if min_index < self.min_track() {
            let index_abs = min_index.abs();
            required_negative_capacity = (index_abs as u16 - self.negative_track_capacity()) as usize;
            new_negative_tracks = (index_abs + self.min_track()) as u16;
        }
        let mut required_positive_capacity = 0;
        let mut new_positive_tracks = 0;
        if max_index > self.max_track() as i16 {
            required_positive_capacity = ((max_index as u16) - self.positive_track_capacity()) as usize;
            new_positive_tracks = (max_index - self.max_track()) as u16;
        }

        // Reserve extra capacity for new elements
        self.tracks.reserve((required_negative_capacity + required_positive_capacity) * 2);

        // Make space for new negative tracks by pushing uninit grid tracks then
        // rotating the vector to move those tracks to the beginning of the vector.
        self.tracks.resize(self.tracks.len() + required_negative_capacity, GridTrack::uninit());
        self.tracks.rotate_right(required_negative_capacity);

        // Create new negative tracks
        let mut negative_auto_tracks_iter = auto_tracks.iter().rev().cycle();
        let min_line_index = self.origin - self.track_counts.negative_implicit;
        for i in (min_line_index - 1..(min_line_index - 1 - new_negative_tracks)).into_iter().step_by(2) {
            let track_def = negative_auto_tracks_iter.next().unwrap_or(&TrackSizingFunction::Auto);
            self.tracks[i as usize] = GridTrack::new(track_def.min_sizing_function(), track_def.max_sizing_function());
            self.tracks[(i - 1) as usize] = GridTrack::gutter(gap);
        }

        // Create new positive tracks
        let mut positive_auto_tracks_iter = auto_tracks.iter().cycle();
        for _ in (0..new_positive_tracks).into_iter() {
            let track_def = positive_auto_tracks_iter.next().unwrap_or(&TrackSizingFunction::Auto);
            self.tracks.push(GridTrack::new(track_def.min_sizing_function(), track_def.max_sizing_function()));
            self.tracks.push(GridTrack::gutter(gap));
        }
    }

    /// The sum of the flex factors (fr units) of the flexible tracks.
    /// If this value is less than 1, set it to 1 instead.
    fn flex_factor_sum(&self) -> f32 {
        self.tracks.iter().map(|track| track.flex_factor()).sum::<f32>().max(1.0)
    }

    /// The space to fill minus the base sizes of the non-flexible grid tracks.
    fn leftover_space(&self) -> f32 {
        self.tracks.iter().filter(|track| !track.is_flexible()).map(|track| track.base_size).sum()
    }

    /// Let the hypothetical fr size be the leftover space divided by the flex factor sum.
    fn hypothetical_fr_size(&self) -> f32 {
        self.leftover_space() / self.flex_factor_sum()
    }
}

// pub(super) enum GridPosition {
//     Auto,
//     LineIndex(i16),
//     LineName(u16),
//     // GridAreaStart(u16),
//     // GridAreaEnd(u16),
// }

pub(super) struct NamedArea {
    name: u16,
    row_start: u16,
    row_end: u16,
    column_start: u16,
    column_end: u16,
}

#[derive(Debug)]
pub(in super::super) struct GridItem {
    /// The id of the Node that this item represents
    pub node: Node,

    /// The item's definite row-start and row-end, as resolved by the placement algorithm
    /// (in origin-zero coordinates)
    pub row: Line<i16>,
    /// The items definite column-start and column-end, as resolved by the placement algorithm
    /// (in origin-zero coordinates)
    pub column: Line<i16>,

    /// The item's definite row-start and row-end (same as `row` field, except in a different coordinate system)
    /// (as indexes into the Vec<GridTrack> stored in a grid's GridAxisTracks)
    pub row_indexes: Line<u16>,
    /// The items definite column-start and column-end (same as `column` field, except in a different coordinate system)
    /// (as indexes into the Vec<GridTrack> stored in a grid's GridAxisTracks)
    pub column_indexes: Line<u16>,

    /// Whether the item crosses a flexible row
    pub crosses_flexible_row: bool,
    /// Whether the item crosses a flexible column
    pub crosses_flexible_column: bool,

    /// The order of the item in the children array (this is significant for auto-placement!)
    // pub source_order: u16,

    // Caches for intrinsic size computation
    pub known_dimensions_cache: Option<Size<Option<f32>>>,
    pub min_content_contribution_cache: Option<Size<f32>>,
    pub minimum_contribution_cache: Option<f32>,
    pub max_content_contribution_cache: Option<Size<f32>>,

    /// Cache for intrinsic size computation
    /// There is an entry for each combination of (min-content, max-content) and (height, width)
    intrinsic_size_cache: [AvailableSpaceCache; 4],
}

impl GridItem {
    pub fn new_with_placement(node: Node, col_span: Line<i16>, row_span: Line<i16>) -> Self {
        GridItem {
            node,
            row: row_span,
            column: col_span,
            row_indexes: Line { start: 0, end: 0 }, // Properly initialised later
            column_indexes: Line { start: 0, end: 0 }, // Properly initialised later
            crosses_flexible_row: false,            // Properly initialised later
            crosses_flexible_column: false,         // Properly initialised later
            known_dimensions_cache: None,
            min_content_contribution_cache: None,
            max_content_contribution_cache: None,
            minimum_contribution_cache: None,
            intrinsic_size_cache: [AvailableSpaceCache::empty(); 4], // source_order: 1,
        }
    }

    pub fn placement(&self, axis: GridAxis) -> Line<i16> {
        match axis {
            GridAxis::Block => self.row,
            GridAxis::Inline => self.column,
        }
    }

    pub fn placement_indexes(&self, axis: GridAxis) -> Line<u16> {
        match axis {
            GridAxis::Block => self.row_indexes,
            GridAxis::Inline => self.column_indexes,
        }
    }

    pub fn track_range_excluding_lines(&self, axis: GridAxis) -> Range<usize> {
        let indexes = self.placement_indexes(axis);
        (indexes.start as usize + 1)..(indexes.end as usize)
    }

    pub fn span(&self, axis: GridAxis) -> u16 {
        match axis {
            GridAxis::Block => match (&self.row.start, &self.row.end) {
                (start, end) => max(end - start, 0) as u16,
                _ => 0,
            },
            GridAxis::Inline => match (&self.column.start, &self.column.end) {
                (start, end) => max(end - start, 0) as u16,
                _ => 0,
            },
        }
    }

    pub fn crosses_flexible_track(&self, axis: GridAxis) -> bool {
        match axis {
            GridAxis::Inline => self.crosses_flexible_column,
            GridAxis::Block => self.crosses_flexible_row,
        }
    }

    fn cache_entry_index(&self, constraint: Size<AvailableSpace>) -> Option<usize> {
        use AvailableSpace::*;
        match (constraint.width, constraint.height) {
            (MinContent, Definite(_)) => Some(0),
            (MaxContent, Definite(_)) => Some(1),
            (Definite(_), MinContent) => Some(2),
            (Definite(_), MaxContent) => Some(3),
            _ => None,
        }
    }

    fn get_cache(&self, constraint: Size<AvailableSpace>) -> Option<AvailableSpaceCache> {
        self.cache_entry_index(constraint)
            .map(|index| self.intrinsic_size_cache[index])
            .filter(|cache| cache.constraint == constraint)
    }

    fn set_cache(&mut self, constraint: Size<AvailableSpace>, size: Size<f32>) {
        if let Some(index) = self.cache_entry_index(constraint) {
            self.intrinsic_size_cache[index] = AvailableSpaceCache { constraint, cached_size: size }
        }
    }

    pub fn known_dimensions_cached(
        &mut self,
        axis: GridAxis,
        other_axis_tracks: &[GridTrack],
        other_axis_available_space: AvailableSpace,
        get_track_size_estimate: impl Fn(&GridTrack, AvailableSpace) -> Option<f32>,
    ) -> Size<Option<f32>> {
        self.known_dimensions_cache.unwrap_or_else(|| {
            let item_other_axis_size: Option<f32> = {
                (&other_axis_tracks)[self.track_range_excluding_lines(axis.other())]
                    .iter()
                    .map(|track| get_track_size_estimate(track, other_axis_available_space))
                    .sum::<Option<f32>>()
            };
            let known_dimensions = {
                let mut size = Size::NONE;
                size.set(axis.other(), item_other_axis_size);
                size
            };

            self.known_dimensions_cache = Some(known_dimensions);

            known_dimensions
        })
    }

    pub fn min_content_contribution_cached(
        &mut self,
        tree: &mut impl LayoutTree,
        known_dimensions: Size<Option<f32>>,
    ) -> Size<f32> {
        self.min_content_contribution_cache.unwrap_or_else(|| {
            let size = compute_node_layout(
                tree,
                self.node,
                known_dimensions,
                Size::MIN_CONTENT,
                RunMode::ComputeSize,
                SizingMode::InherentSize,
            );
            self.min_content_contribution_cache = Some(size);
            size
        })
    }

    pub fn max_content_contribution_cached(
        &mut self,
        tree: &mut impl LayoutTree,
        known_dimensions: Size<Option<f32>>,
    ) -> Size<f32> {
        self.max_content_contribution_cache.unwrap_or_else(|| {
            let size = compute_node_layout(
                tree,
                self.node,
                known_dimensions,
                Size::MAX_CONTENT,
                RunMode::ComputeSize,
                SizingMode::InherentSize,
            );
            self.min_content_contribution_cache = Some(size);
            size
        })
    }

    // The minimum contribution of an item is the smallest outer size it can have.
    // Specifically:
    //   - If the item’s computed preferred size behaves as auto or depends on the size of its containing block in the relevant axis:
    //     Its minimum contribution is the outer size that would result from assuming the item’s used minimum size as its preferred size;
    //   - Else the item’s minimum contribution is its min-content contribution.
    // Because the minimum contribution often depends on the size of the item’s content, it is considered a type of intrinsic size contribution.
    pub fn minimum_contribution_cached(
        &mut self,
        tree: &mut impl LayoutTree,
        axis: GridAxis,
        axis_tracks: &[GridTrack],
        available_space: Size<AvailableSpace>,
        known_dimensions: Size<Option<f32>>,
    ) -> f32 {
        self.minimum_contribution_cache.unwrap_or_else(|| {
            let style = tree.style(self.node);
            let axis_available_space = available_space.get(axis).into_option();
            style
                .size
                .get(axis)
                .maybe_resolve(axis_available_space)
                .or_else(|| style.min_size.get(axis).maybe_resolve(axis_available_space))
                .unwrap_or_else(|| {
                    // Automatic minimum size. See https://www.w3.org/TR/css-grid-1/#min-size-auto

                    // To provide a more reasonable default minimum size for grid items, the used value of its automatic minimum size
                    // in a given axis is the content-based minimum size if all of the following are true:
                    let item_axis_tracks = &axis_tracks[self.track_range_excluding_lines(axis)];

                    // it is not a scroll container
                    // TODO: support overflow propety

                    // it spans at least one track in that axis whose min track sizing function is auto
                    let spans_auto_min_track = axis_tracks
                        .iter()
                        // TODO: should this be 'behaves as auto' rather than just literal auto?
                        .any(|track| track.min_track_sizing_function == MinTrackSizingFunction::Auto);

                    // if it spans more than one track in that axis, none of those tracks are flexible
                    let only_span_one_track = item_axis_tracks.len() == 1;
                    let spans_a_flexible_track = axis_tracks
                        .iter()
                        .any(|track| matches!(track.max_track_sizing_function, MaxTrackSizingFunction::Flex(_)));

                    let use_content_based_minimum =
                        spans_auto_min_track && (only_span_one_track || !spans_a_flexible_track);

                    // Otherwise, the automatic minimum size is zero, as usual.
                    if use_content_based_minimum {
                        self.min_content_contribution_cached(tree, known_dimensions).get(axis)
                    } else {
                        0.0
                    }
                })
        })
    }

    pub fn clear_contribution_caches(&mut self) {
        self.known_dimensions_cache = None;
        self.min_content_contribution_cache = None;
        self.max_content_contribution_cache = None;
        self.minimum_contribution_cache = None;
    }

    pub fn intrinsic_size_cached(
        &mut self,
        measure_node: impl Fn(Node, Size<AvailableSpace>) -> Size<f32>,
        constraint: Size<AvailableSpace>,
    ) -> Size<f32> {
        if let Some(cache) = self.get_cache(constraint) {
            cache.cached_size
        } else {
            let size = measure_node(self.node, constraint);
            self.set_cache(constraint, size);
            size
        }
    }

    pub fn axis_agnostic_intrinsic_size_cached(
        &mut self,
        measure_node: impl Fn(Node, Size<AvailableSpace>) -> Size<f32>,
        axis: GridAxis,
        constraint: Size<AvailableSpace>,
        other_axis_constraint: Size<AvailableSpace>,
    ) -> Size<f32> {
        if let Some(cache) = self.get_cache(constraint) {
            cache.cached_size
        } else {
            let size = measure_node(self.node, constraint);
            self.set_cache(constraint, size);
            size
        }
    }
}

pub(super) struct CssGrid {
    pub available_space: Size<AvailableSpace>,
    pub cell_occupancy_matrix: CellOccupancyMatrix,
    pub items: Vec<GridItem>,
    pub columns: GridAxisTracks,
    pub rows: GridAxisTracks,
    pub named_areas: Vec<NamedArea>,
}
