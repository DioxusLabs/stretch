pub fn compute() {
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_with_children(
            taffy::style::FlexboxLayout {
                size: taffy::geometry::Size {
                    width: Some(taffy::style::Dimension::Percent(0.5f32)),
                    height: Some(taffy::style::Dimension::Percent(0.5f32)),
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
                flex_direction: taffy::style::FlexDirection::Column,
                size: taffy::geometry::Size {
                    width: Some(taffy::style::Dimension::Points(200f32)),
                    height: Some(taffy::style::Dimension::Points(400f32)),
                    ..Default::default()
                },
                padding: taffy::geometry::Rect {
                    start: Some(taffy::style::Dimension::Points(20f32)),
                    end: Some(taffy::style::Dimension::Points(20f32)),
                    top: Some(taffy::style::Dimension::Points(20f32)),
                    bottom: Some(taffy::style::Dimension::Points(20f32)),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();
}
