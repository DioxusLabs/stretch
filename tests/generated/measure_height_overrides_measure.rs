#[test]
fn measure_height_overrides_measure() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout};
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy.new_leaf_with_measure(
        taffy::style::Style {
            size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Length(5f32) },
            ..Default::default()
        },
        taffy::tree::MeasureFunc::Raw(|known_dimensions, available_space| {
            const TEXT: &str = "H";
            super::measure_standard_text(known_dimensions, available_space, TEXT, super::WritingMode::Horizontal, None)
        }),
    );
    let node = taffy.new_with_children(taffy::style::Style { ..Default::default() }, &[node0]);
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT);
    println!("\nComputed tree:");
    taffy::util::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node);
    assert_eq!(size.width, 10f32, "width of node {:?}. Expected {}. Actual {}", node, 10f32, size.width);
    assert_eq!(size.height, 5f32, "height of node {:?}. Expected {}. Actual {}", node, 5f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0);
    assert_eq!(size.width, 10f32, "width of node {:?}. Expected {}. Actual {}", node0, 10f32, size.width);
    assert_eq!(size.height, 5f32, "height of node {:?}. Expected {}. Actual {}", node0, 5f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
}
