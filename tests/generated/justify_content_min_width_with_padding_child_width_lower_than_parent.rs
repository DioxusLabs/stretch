#[test]
fn justify_content_min_width_with_padding_child_width_lower_than_parent() {
    use slotmap::Key;
    #[allow(unused_imports)]
    use taffy::{layout::Layout, prelude::*};
    let mut taffy = taffy::Taffy::new();
    let node000 = taffy
        .new_leaf(taffy::style::Style {
            align_content: Some(taffy::style::AlignContent::Stretch),
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(199f32),
                height: taffy::style::Dimension::Points(100f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node00 = taffy
        .new_with_children(
            taffy::style::Style {
                align_content: Some(taffy::style::AlignContent::Stretch),
                justify_content: Some(taffy::style::JustifyContent::Center),
                min_size: taffy::geometry::Size { width: taffy::style::Dimension::Points(400f32), height: auto() },
                padding: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentage::Points(100f32),
                    right: taffy::style::LengthPercentage::Points(100f32),
                    top: zero(),
                    bottom: zero(),
                },
                ..Default::default()
            },
            &[node000],
        )
        .unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style { align_content: Some(taffy::style::AlignContent::Stretch), ..Default::default() },
            &[node00],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                flex_direction: taffy::style::FlexDirection::Column,
                align_content: Some(taffy::style::AlignContent::Stretch),
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(1080f32),
                    height: taffy::style::Dimension::Points(1584f32),
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    println!("\nComputed tree:");
    taffy::debug::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 1080f32, "width of node {:?}. Expected {}. Actual {}", node.data(), 1080f32, size.width);
    assert_eq!(size.height, 1584f32, "height of node {:?}. Expected {}. Actual {}", node.data(), 1584f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node.data(), 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 1080f32, "width of node {:?}. Expected {}. Actual {}", node0.data(), 1080f32, size.width);
    assert_eq!(size.height, 100f32, "height of node {:?}. Expected {}. Actual {}", node0.data(), 100f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0.data(), 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node00).unwrap();
    assert_eq!(size.width, 400f32, "width of node {:?}. Expected {}. Actual {}", node00.data(), 400f32, size.width);
    assert_eq!(size.height, 100f32, "height of node {:?}. Expected {}. Actual {}", node00.data(), 100f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node00.data(), 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node00.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node000).unwrap();
    assert_eq!(size.width, 199f32, "width of node {:?}. Expected {}. Actual {}", node000.data(), 199f32, size.width);
    assert_eq!(size.height, 100f32, "height of node {:?}. Expected {}. Actual {}", node000.data(), 100f32, size.height);
    assert_eq!(location.x, 101f32, "x of node {:?}. Expected {}. Actual {}", node000.data(), 101f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node000.data(), 0f32, location.y);
}
