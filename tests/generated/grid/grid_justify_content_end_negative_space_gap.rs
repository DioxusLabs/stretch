#[test]
fn grid_justify_content_end_negative_space_gap() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout};
    let mut taffy = taffy::Taffy::new();
    let node00 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node01 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node02 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node03 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node04 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node05 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node06 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node07 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node08 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                align_content: Some(taffy::style::AlignContent::Center),
                justify_content: Some(taffy::style::JustifyContent::End),
                gap: taffy::geometry::Size {
                    width: taffy::style::LengthPercentage::Length(10f32),
                    height: taffy::style::LengthPercentage::Length(10f32),
                },
                grid_template_rows: vec![length(20f32), length(20f32), length(20f32)],
                grid_template_columns: vec![length(40f32), length(40f32), length(40f32)],
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Length(120f32),
                    height: taffy::style::Dimension::Length(120f32),
                },
                ..Default::default()
            },
            &[node00, node01, node02, node03, node04, node05, node06, node07, node08],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Length(240f32),
                    height: taffy::style::Dimension::Length(240f32),
                },
                border: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentage::Length(60f32),
                    right: taffy::style::LengthPercentage::Length(60f32),
                    top: taffy::style::LengthPercentage::Length(60f32),
                    bottom: taffy::style::LengthPercentage::Length(60f32),
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    println!("\nComputed tree:");
    taffy::util::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 240f32, "width of node {:?}. Expected {}. Actual {}", node, 240f32, size.width);
    assert_eq!(size.height, 240f32, "height of node {:?}. Expected {}. Actual {}", node, 240f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 120f32, "width of node {:?}. Expected {}. Actual {}", node0, 120f32, size.width);
    assert_eq!(size.height, 120f32, "height of node {:?}. Expected {}. Actual {}", node0, 120f32, size.height);
    assert_eq!(location.x, 60f32, "x of node {:?}. Expected {}. Actual {}", node0, 60f32, location.x);
    assert_eq!(location.y, 60f32, "y of node {:?}. Expected {}. Actual {}", node0, 60f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node00).unwrap();
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node00, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node00, 20f32, size.height);
    assert_eq!(location.x, -20f32, "x of node {:?}. Expected {}. Actual {}", node00, -20f32, location.x);
    assert_eq!(location.y, 20f32, "y of node {:?}. Expected {}. Actual {}", node00, 20f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node01).unwrap();
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node01, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node01, 20f32, size.height);
    assert_eq!(location.x, 30f32, "x of node {:?}. Expected {}. Actual {}", node01, 30f32, location.x);
    assert_eq!(location.y, 20f32, "y of node {:?}. Expected {}. Actual {}", node01, 20f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node02).unwrap();
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node02, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node02, 20f32, size.height);
    assert_eq!(location.x, 80f32, "x of node {:?}. Expected {}. Actual {}", node02, 80f32, location.x);
    assert_eq!(location.y, 20f32, "y of node {:?}. Expected {}. Actual {}", node02, 20f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node03).unwrap();
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node03, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node03, 20f32, size.height);
    assert_eq!(location.x, -20f32, "x of node {:?}. Expected {}. Actual {}", node03, -20f32, location.x);
    assert_eq!(location.y, 50f32, "y of node {:?}. Expected {}. Actual {}", node03, 50f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node04).unwrap();
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node04, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node04, 20f32, size.height);
    assert_eq!(location.x, 30f32, "x of node {:?}. Expected {}. Actual {}", node04, 30f32, location.x);
    assert_eq!(location.y, 50f32, "y of node {:?}. Expected {}. Actual {}", node04, 50f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node05).unwrap();
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node05, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node05, 20f32, size.height);
    assert_eq!(location.x, 80f32, "x of node {:?}. Expected {}. Actual {}", node05, 80f32, location.x);
    assert_eq!(location.y, 50f32, "y of node {:?}. Expected {}. Actual {}", node05, 50f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node06).unwrap();
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node06, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node06, 20f32, size.height);
    assert_eq!(location.x, -20f32, "x of node {:?}. Expected {}. Actual {}", node06, -20f32, location.x);
    assert_eq!(location.y, 80f32, "y of node {:?}. Expected {}. Actual {}", node06, 80f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node07).unwrap();
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node07, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node07, 20f32, size.height);
    assert_eq!(location.x, 30f32, "x of node {:?}. Expected {}. Actual {}", node07, 30f32, location.x);
    assert_eq!(location.y, 80f32, "y of node {:?}. Expected {}. Actual {}", node07, 80f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node08).unwrap();
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node08, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node08, 20f32, size.height);
    assert_eq!(location.x, 80f32, "x of node {:?}. Expected {}. Actual {}", node08, 80f32, location.x);
    assert_eq!(location.y, 80f32, "y of node {:?}. Expected {}. Actual {}", node08, 80f32, location.y);
}
