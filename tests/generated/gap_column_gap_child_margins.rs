#[test]
fn gap_column_gap_child_margins() {
    #[allow(unused_imports)]
    use taffy::prelude::*;
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                flex_grow: 1f32,
                flex_shrink: 1f32,
                flex_basis: taffy::style::Dimension::Percent(0f32),
                margin: taffy::geometry::Rect {
                    left: taffy::style::Dimension::Points(2f32),
                    right: taffy::style::Dimension::Points(2f32),
                    ..Rect::zero()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node1 = taffy
        .new_with_children(
            taffy::style::Style {
                flex_grow: 1f32,
                flex_shrink: 1f32,
                flex_basis: taffy::style::Dimension::Percent(0f32),
                margin: taffy::geometry::Rect {
                    left: taffy::style::Dimension::Points(10f32),
                    right: taffy::style::Dimension::Points(10f32),
                    ..Rect::zero()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node2 = taffy
        .new_with_children(
            taffy::style::Style {
                flex_grow: 1f32,
                flex_shrink: 1f32,
                flex_basis: taffy::style::Dimension::Percent(0f32),
                margin: taffy::geometry::Rect {
                    left: taffy::style::Dimension::Points(15f32),
                    right: taffy::style::Dimension::Points(15f32),
                    ..Rect::zero()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                gap: taffy::geometry::Size { width: taffy::style::LengthPercentage::Points(10f32), ..Size::zero() },
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(80f32),
                    height: taffy::style::Dimension::Points(100f32),
                    ..Size::auto()
                },
                ..Default::default()
            },
            &[node0, node1, node2],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    println!("\nComputed tree:");
    taffy::debug::print_tree(&taffy, node);
    println!();
    assert_eq!(taffy.layout(node).unwrap().size.width, 80f32);
    assert_eq!(taffy.layout(node).unwrap().size.height, 100f32);
    assert_eq!(taffy.layout(node).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().size.width, 2f32);
    assert_eq!(taffy.layout(node0).unwrap().size.height, 100f32);
    assert_eq!(taffy.layout(node0).unwrap().location.x, 2f32);
    assert_eq!(taffy.layout(node0).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node1).unwrap().size.width, 2f32);
    assert_eq!(taffy.layout(node1).unwrap().size.height, 100f32);
    assert_eq!(taffy.layout(node1).unwrap().location.x, 26f32);
    assert_eq!(taffy.layout(node1).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node2).unwrap().size.width, 2f32);
    assert_eq!(taffy.layout(node2).unwrap().size.height, 100f32);
    assert_eq!(taffy.layout(node2).unwrap().location.x, 63f32);
    assert_eq!(taffy.layout(node2).unwrap().location.y, 0f32);
}
