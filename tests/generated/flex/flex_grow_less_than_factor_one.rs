#[test]
fn flex_grow_less_than_factor_one() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout, Taffy};
    let mut taffy: Taffy<crate::TextMeasure> = Taffy::new();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            flex_grow: 0.2f32,
            flex_shrink: 0f32,
            flex_basis: taffy::style::Dimension::Length(40f32),
            ..Default::default()
        })
        .unwrap();
    let node1 =
        taffy.new_leaf(taffy::style::Style { flex_grow: 0.2f32, flex_shrink: 0f32, ..Default::default() }).unwrap();
    let node2 =
        taffy.new_leaf(taffy::style::Style { flex_grow: 0.4f32, flex_shrink: 0f32, ..Default::default() }).unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Length(500f32),
                    height: taffy::style::Dimension::Length(200f32),
                },
                ..Default::default()
            },
            &[node0, node1, node2],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    #[cfg_attr(not(feature = "content_size"), allow(unused_variables))]
    let layout @ Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 500f32, "width of node {:?}. Expected {}. Actual {}", node, 500f32, size.width);
    assert_eq!(size.height, 200f32, "height of node {:?}. Expected {}. Actual {}", node, 200f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_width(),
        0f32,
        "scroll_width of node {:?}. Expected {}. Actual {}",
        node,
        0f32,
        layout.scroll_width()
    );
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_height(),
        0f32,
        "scroll_height of node {:?}. Expected {}. Actual {}",
        node,
        0f32,
        layout.scroll_height()
    );
    #[cfg_attr(not(feature = "content_size"), allow(unused_variables))]
    let layout @ Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 132f32, "width of node {:?}. Expected {}. Actual {}", node0, 132f32, size.width);
    assert_eq!(size.height, 200f32, "height of node {:?}. Expected {}. Actual {}", node0, 200f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_width(),
        0f32,
        "scroll_width of node {:?}. Expected {}. Actual {}",
        node0,
        0f32,
        layout.scroll_width()
    );
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_height(),
        0f32,
        "scroll_height of node {:?}. Expected {}. Actual {}",
        node0,
        0f32,
        layout.scroll_height()
    );
    #[cfg_attr(not(feature = "content_size"), allow(unused_variables))]
    let layout @ Layout { size, location, .. } = taffy.layout(node1).unwrap();
    assert_eq!(size.width, 92f32, "width of node {:?}. Expected {}. Actual {}", node1, 92f32, size.width);
    assert_eq!(size.height, 200f32, "height of node {:?}. Expected {}. Actual {}", node1, 200f32, size.height);
    assert_eq!(location.x, 132f32, "x of node {:?}. Expected {}. Actual {}", node1, 132f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node1, 0f32, location.y);
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_width(),
        0f32,
        "scroll_width of node {:?}. Expected {}. Actual {}",
        node1,
        0f32,
        layout.scroll_width()
    );
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_height(),
        0f32,
        "scroll_height of node {:?}. Expected {}. Actual {}",
        node1,
        0f32,
        layout.scroll_height()
    );
    #[cfg_attr(not(feature = "content_size"), allow(unused_variables))]
    let layout @ Layout { size, location, .. } = taffy.layout(node2).unwrap();
    assert_eq!(size.width, 184f32, "width of node {:?}. Expected {}. Actual {}", node2, 184f32, size.width);
    assert_eq!(size.height, 200f32, "height of node {:?}. Expected {}. Actual {}", node2, 200f32, size.height);
    assert_eq!(location.x, 224f32, "x of node {:?}. Expected {}. Actual {}", node2, 224f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node2, 0f32, location.y);
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_width(),
        0f32,
        "scroll_width of node {:?}. Expected {}. Actual {}",
        node2,
        0f32,
        layout.scroll_width()
    );
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_height(),
        0f32,
        "scroll_height of node {:?}. Expected {}. Actual {}",
        node2,
        0f32,
        layout.scroll_height()
    );
}
