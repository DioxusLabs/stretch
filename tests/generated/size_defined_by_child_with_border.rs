#[test]
fn size_defined_by_child_with_border() {
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_with_children(
            taffy::style::FlexboxLayout {
                size: taffy::geometry::Size {
                    width: Some(taffy::style::Dimension::Points(10f32)),
                    height: Some(taffy::style::Dimension::Points(10f32)),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::FlexboxLayout {
                border: taffy::geometry::Rect {
                    start: Some(taffy::style::Dimension::Points(10f32)),
                    end: Some(taffy::style::Dimension::Points(10f32)),
                    top: Some(taffy::style::Dimension::Points(10f32)),
                    bottom: Some(taffy::style::Dimension::Points(10f32)),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();
    assert_eq!(taffy.layout(node).unwrap().size.width, 30f32);
    assert_eq!(taffy.layout(node).unwrap().size.height, 30f32);
    assert_eq!(taffy.layout(node).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().size.width, 10f32);
    assert_eq!(taffy.layout(node0).unwrap().size.height, 10f32);
    assert_eq!(taffy.layout(node0).unwrap().location.x, 10f32);
    assert_eq!(taffy.layout(node0).unwrap().location.y, 10f32);
}
