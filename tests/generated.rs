#[cfg(test)]
mod generated {
    #[test]
    fn justify_content_row_space_around() {
        let layout = stretch::compute(&stretch::style::Node {
            justify_content: stretch::style::JustifyContent::SpaceAround,
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![
                stretch::style::Node { width: stretch::style::Dimension::Points(10.0), ..Default::default() },
                stretch::style::Node { width: stretch::style::Dimension::Points(10.0), ..Default::default() },
                stretch::style::Node { width: stretch::style::Dimension::Points(10.0), ..Default::default() },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 10.0);
        assert_eq!(layout.children[0].height, 100.0);
        assert_eq!(layout.children[0].x, 12.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 10.0);
        assert_eq!(layout.children[1].height, 100.0);
        assert_eq!(layout.children[1].x, 45.0);
        assert_eq!(layout.children[1].y, 0.0);

        assert_eq!(layout.children[2].width, 10.0);
        assert_eq!(layout.children[2].height, 100.0);
        assert_eq!(layout.children[2].x, 78.0);
        assert_eq!(layout.children[2].y, 0.0);
    }

    #[test]
    fn align_items_center() {
        let layout = stretch::compute(&stretch::style::Node {
            align_items: stretch::style::AlignItems::Center,
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![stretch::style::Node {
                width: stretch::style::Dimension::Points(10.0),
                height: stretch::style::Dimension::Points(10.0),
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 10.0);
        assert_eq!(layout.children[0].height, 10.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 45.0);
    }

    #[test]
    fn absolute_layout_justify_content_center() {
        let layout = stretch::compute(&stretch::style::Node {
            justify_content: stretch::style::JustifyContent::Center,
            width: stretch::style::Dimension::Points(110.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![stretch::style::Node {
                position: stretch::style::Position::Absolute,
                width: stretch::style::Dimension::Points(60.0),
                height: stretch::style::Dimension::Points(40.0),
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 110.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 60.0);
        assert_eq!(layout.children[0].height, 40.0);
        assert_eq!(layout.children[0].x, 25.0);
        assert_eq!(layout.children[0].y, 0.0);
    }

    #[test]
    fn margin_auto_bottom_and_top() {
        let layout = stretch::compute(&stretch::style::Node {
            align_items: stretch::style::AlignItems::Center,
            width: stretch::style::Dimension::Points(200.0),
            height: stretch::style::Dimension::Points(200.0),
            children: vec![
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(50.0),
                    height: stretch::style::Dimension::Points(50.0),
                    margin: stretch::style::Edges {
                        top: stretch::style::Dimension::Auto,
                        bottom: stretch::style::Dimension::Auto,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(50.0),
                    height: stretch::style::Dimension::Points(50.0),
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 200.0);
        assert_eq!(layout.height, 200.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 50.0);
        assert_eq!(layout.children[0].height, 50.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 75.0);

        assert_eq!(layout.children[1].width, 50.0);
        assert_eq!(layout.children[1].height, 50.0);
        assert_eq!(layout.children[1].x, 50.0);
        assert_eq!(layout.children[1].y, 75.0);
    }

    #[test]
    fn margin_auto_left_and_right_strech() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(200.0),
            height: stretch::style::Dimension::Points(200.0),
            children: vec![
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(50.0),
                    height: stretch::style::Dimension::Points(50.0),
                    margin: stretch::style::Edges {
                        start: stretch::style::Dimension::Auto,
                        end: stretch::style::Dimension::Auto,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(50.0),
                    height: stretch::style::Dimension::Points(50.0),
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 200.0);
        assert_eq!(layout.height, 200.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 50.0);
        assert_eq!(layout.children[0].height, 50.0);
        assert_eq!(layout.children[0].x, 50.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 50.0);
        assert_eq!(layout.children[1].height, 50.0);
        assert_eq!(layout.children[1].x, 150.0);
        assert_eq!(layout.children[1].y, 0.0);
    }

    #[test]
    fn rounding_flex_basis_overrides_main_size() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(113.0),
            children: vec![
                stretch::style::Node {
                    flex_grow: 1.0,
                    flex_basis: stretch::style::Dimension::Points(50.0),
                    height: stretch::style::Dimension::Points(20.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 1.0,
                    height: stretch::style::Dimension::Points(10.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 1.0,
                    height: stretch::style::Dimension::Points(10.0),
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 113.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 100.0);
        assert_eq!(layout.children[0].height, 64.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 100.0);
        assert_eq!(layout.children[1].height, 25.0);
        assert_eq!(layout.children[1].x, 0.0);
        assert_eq!(layout.children[1].y, 64.0);

        assert_eq!(layout.children[2].width, 100.0);
        assert_eq!(layout.children[2].height, 24.0);
        assert_eq!(layout.children[2].x, 0.0);
        assert_eq!(layout.children[2].y, 89.0);
    }

    #[test]
    fn percentage_flex_basis() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(200.0),
            height: stretch::style::Dimension::Points(200.0),
            children: vec![
                stretch::style::Node {
                    flex_grow: 1.0,
                    flex_basis: stretch::style::Dimension::Percent(0.5),
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 1.0,
                    flex_basis: stretch::style::Dimension::Percent(0.2),
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 200.0);
        assert_eq!(layout.height, 200.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 125.0);
        assert_eq!(layout.children[0].height, 200.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 75.0);
        assert_eq!(layout.children[1].height, 200.0);
        assert_eq!(layout.children[1].x, 125.0);
        assert_eq!(layout.children[1].y, 0.0);
    }

    #[test]
    fn rounding_fractial_input_3() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(113.4),
            children: vec![
                stretch::style::Node {
                    flex_grow: 1.0,
                    flex_basis: stretch::style::Dimension::Points(50.0),
                    height: stretch::style::Dimension::Points(20.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 1.0,
                    height: stretch::style::Dimension::Points(10.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 1.0,
                    height: stretch::style::Dimension::Points(10.0),
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 113.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 100.0);
        assert_eq!(layout.children[0].height, 64.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 100.0);
        assert_eq!(layout.children[1].height, 25.0);
        assert_eq!(layout.children[1].x, 0.0);
        assert_eq!(layout.children[1].y, 64.0);

        assert_eq!(layout.children[2].width, 100.0);
        assert_eq!(layout.children[2].height, 24.0);
        assert_eq!(layout.children[2].x, 0.0);
        assert_eq!(layout.children[2].y, 89.0);
    }

    #[test]
    fn margin_and_stretch_column() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![stretch::style::Node {
                flex_grow: 1.0,
                margin: stretch::style::Edges {
                    start: stretch::style::Dimension::Points(10.0),
                    end: stretch::style::Dimension::Points(10.0),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 80.0);
        assert_eq!(layout.children[0].height, 100.0);
        assert_eq!(layout.children[0].x, 10.0);
        assert_eq!(layout.children[0].y, 0.0);
    }

    #[test]
    fn wrapped_column_max_height_flex() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            flexWrap: stretch::style::FlexWrap::Wrap,
            align_items: stretch::style::AlignItems::Center,
            align_content: stretch::style::AlignContent::Center,
            justify_content: stretch::style::JustifyContent::Center,
            width: stretch::style::Dimension::Points(700.0),
            height: stretch::style::Dimension::Points(500.0),
            children: vec![
                stretch::style::Node {
                    flex_grow: 1.0,
                    flex_shrink: 1.0,
                    flex_basis: stretch::style::Dimension::Percent(0.0),
                    width: stretch::style::Dimension::Points(100.0),
                    height: stretch::style::Dimension::Points(500.0),
                    max_height: stretch::style::Dimension::Points(200.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 1.0,
                    flex_shrink: 1.0,
                    flex_basis: stretch::style::Dimension::Percent(0.0),
                    width: stretch::style::Dimension::Points(200.0),
                    height: stretch::style::Dimension::Points(200.0),
                    margin: stretch::style::Edges {
                        start: stretch::style::Dimension::Points(20.0),
                        end: stretch::style::Dimension::Points(20.0),
                        top: stretch::style::Dimension::Points(20.0),
                        bottom: stretch::style::Dimension::Points(20.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(100.0),
                    height: stretch::style::Dimension::Points(100.0),
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 700.0);
        assert_eq!(layout.height, 500.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 100.0);
        assert_eq!(layout.children[0].height, 180.0);
        assert_eq!(layout.children[0].x, 300.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 200.0);
        assert_eq!(layout.children[1].height, 180.0);
        assert_eq!(layout.children[1].x, 250.0);
        assert_eq!(layout.children[1].y, 200.0);

        assert_eq!(layout.children[2].width, 100.0);
        assert_eq!(layout.children[2].height, 100.0);
        assert_eq!(layout.children[2].x, 300.0);
        assert_eq!(layout.children[2].y, 400.0);
    }

    #[test]
    fn align_baseline_nested_child() {
        let layout = stretch::compute(&stretch::style::Node {
            align_items: stretch::style::AlignItems::Baseline,
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(50.0),
                    height: stretch::style::Dimension::Points(50.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_direction: stretch::style::FlexDirection::Column,
                    width: stretch::style::Dimension::Points(50.0),
                    height: stretch::style::Dimension::Points(20.0),
                    children: vec![stretch::style::Node {
                        width: stretch::style::Dimension::Points(50.0),
                        height: stretch::style::Dimension::Points(10.0),
                        ..Default::default()
                    }],
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 50.0);
        assert_eq!(layout.children[0].height, 50.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 50.0);
        assert_eq!(layout.children[1].height, 20.0);
        assert_eq!(layout.children[1].x, 50.0);
        assert_eq!(layout.children[1].y, 40.0);

        assert_eq!(layout.children[1].children[0].width, 50.0);
        assert_eq!(layout.children[1].children[0].height, 10.0);
        assert_eq!(layout.children[1].children[0].x, 0.0);
        assert_eq!(layout.children[1].children[0].y, 0.0);
    }

    #[test]
    fn max_height_overrides_height_on_root() {
        let layout = stretch::compute(&stretch::style::Node {
            height: stretch::style::Dimension::Points(200.0),
            max_height: stretch::style::Dimension::Points(100.0),
            ..Default::default()
        });

        assert_eq!(layout.width, 0.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);
    }

    #[test]
    fn padding_center_child() {
        let layout = stretch::compute(&stretch::style::Node {
            align_items: stretch::style::AlignItems::Center,
            justify_content: stretch::style::JustifyContent::Center,
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            padding: stretch::style::Edges {
                start: stretch::style::Dimension::Points(10.0),
                end: stretch::style::Dimension::Points(20.0),
                top: stretch::style::Dimension::Points(10.0),
                bottom: stretch::style::Dimension::Points(20.0),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                width: stretch::style::Dimension::Points(10.0),
                height: stretch::style::Dimension::Points(10.0),
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 10.0);
        assert_eq!(layout.children[0].height, 10.0);
        assert_eq!(layout.children[0].x, 40.0);
        assert_eq!(layout.children[0].y, 40.0);
    }

    #[test]
    fn align_flex_start_with_stretching_children() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(500.0),
            height: stretch::style::Dimension::Points(500.0),
            children: vec![stretch::style::Node {
                children: vec![stretch::style::Node {
                    flex_grow: 1.0,
                    flex_shrink: 1.0,
                    children: vec![stretch::style::Node { flex_grow: 1.0, flex_shrink: 1.0, ..Default::default() }],
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 500.0);
        assert_eq!(layout.height, 500.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 0.0);
        assert_eq!(layout.children[0].height, 500.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[0].children[0].width, 0.0);
        assert_eq!(layout.children[0].children[0].height, 500.0);
        assert_eq!(layout.children[0].children[0].x, 0.0);
        assert_eq!(layout.children[0].children[0].y, 0.0);

        assert_eq!(layout.children[0].children[0].children[0].width, 0.0);
        assert_eq!(layout.children[0].children[0].children[0].height, 500.0);
        assert_eq!(layout.children[0].children[0].children[0].x, 0.0);
        assert_eq!(layout.children[0].children[0].children[0].y, 0.0);
    }

    #[test]
    fn min_width_overrides_width() {
        let layout = stretch::compute(&stretch::style::Node {
            children: vec![stretch::style::Node {
                width: stretch::style::Dimension::Points(50.0),
                min_width: stretch::style::Dimension::Points(100.0),
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 0.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 100.0);
        assert_eq!(layout.children[0].height, 0.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);
    }

    #[test]
    fn flex_wrap_align_stretch_fits_one_row() {
        let layout = stretch::compute(&stretch::style::Node {
            flexWrap: stretch::style::FlexWrap::Wrap,
            width: stretch::style::Dimension::Points(150.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![
                stretch::style::Node { width: stretch::style::Dimension::Points(50.0), ..Default::default() },
                stretch::style::Node { width: stretch::style::Dimension::Points(50.0), ..Default::default() },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 150.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 50.0);
        assert_eq!(layout.children[0].height, 100.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 50.0);
        assert_eq!(layout.children[1].height, 100.0);
        assert_eq!(layout.children[1].x, 50.0);
        assert_eq!(layout.children[1].y, 0.0);
    }

    #[test]
    fn justify_content_row_space_evenly() {
        let layout = stretch::compute(&stretch::style::Node {
            justify_content: stretch::style::JustifyContent::SpaceEvenly,
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![
                stretch::style::Node { height: stretch::style::Dimension::Points(10.0), ..Default::default() },
                stretch::style::Node { height: stretch::style::Dimension::Points(10.0), ..Default::default() },
                stretch::style::Node { height: stretch::style::Dimension::Points(10.0), ..Default::default() },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 0.0);
        assert_eq!(layout.children[0].height, 10.0);
        assert_eq!(layout.children[0].x, 25.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 0.0);
        assert_eq!(layout.children[1].height, 10.0);
        assert_eq!(layout.children[1].x, 50.0);
        assert_eq!(layout.children[1].y, 0.0);

        assert_eq!(layout.children[2].width, 0.0);
        assert_eq!(layout.children[2].height, 10.0);
        assert_eq!(layout.children[2].x, 75.0);
        assert_eq!(layout.children[2].y, 0.0);
    }

    #[test]
    fn absolute_layout_percentage_bottom_based_on_parent_height() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(200.0),
            children: vec![
                stretch::style::Node {
                    position: stretch::style::Position::Absolute,
                    width: stretch::style::Dimension::Points(10.0),
                    height: stretch::style::Dimension::Points(10.0),
                    top: stretch::style::Dimension::Percent(0.5),
                    ..Default::default()
                },
                stretch::style::Node {
                    position: stretch::style::Position::Absolute,
                    width: stretch::style::Dimension::Points(10.0),
                    height: stretch::style::Dimension::Points(10.0),
                    bottom: stretch::style::Dimension::Percent(0.5),
                    ..Default::default()
                },
                stretch::style::Node {
                    position: stretch::style::Position::Absolute,
                    width: stretch::style::Dimension::Points(10.0),
                    top: stretch::style::Dimension::Percent(0.1),
                    bottom: stretch::style::Dimension::Percent(0.1),
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 200.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 10.0);
        assert_eq!(layout.children[0].height, 10.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 100.0);

        assert_eq!(layout.children[1].width, 10.0);
        assert_eq!(layout.children[1].height, 10.0);
        assert_eq!(layout.children[1].x, 0.0);
        assert_eq!(layout.children[1].y, 90.0);

        assert_eq!(layout.children[2].width, 10.0);
        assert_eq!(layout.children[2].height, 160.0);
        assert_eq!(layout.children[2].x, 0.0);
        assert_eq!(layout.children[2].y, 20.0);
    }

    #[test]
    fn absolute_layout_align_items_and_justify_content_flex_end() {
        let layout = stretch::compute(&stretch::style::Node {
            align_items: stretch::style::AlignItems::FlexEnd,
            justify_content: stretch::style::JustifyContent::FlexEnd,
            width: stretch::style::Dimension::Points(110.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![stretch::style::Node {
                position: stretch::style::Position::Absolute,
                width: stretch::style::Dimension::Points(60.0),
                height: stretch::style::Dimension::Points(40.0),
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 110.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 60.0);
        assert_eq!(layout.children[0].height, 40.0);
        assert_eq!(layout.children[0].x, 50.0);
        assert_eq!(layout.children[0].y, 60.0);
    }

    #[test]
    fn nested_overflowing_child() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![stretch::style::Node {
                children: vec![stretch::style::Node {
                    width: stretch::style::Dimension::Points(200.0),
                    height: stretch::style::Dimension::Points(200.0),
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 200.0);
        assert_eq!(layout.children[0].height, 100.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[0].children[0].width, 200.0);
        assert_eq!(layout.children[0].children[0].height, 200.0);
        assert_eq!(layout.children[0].children[0].x, 0.0);
        assert_eq!(layout.children[0].children[0].y, 0.0);
    }

    #[test]
    fn flex_direction_row() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![
                stretch::style::Node { width: stretch::style::Dimension::Points(10.0), ..Default::default() },
                stretch::style::Node { width: stretch::style::Dimension::Points(10.0), ..Default::default() },
                stretch::style::Node { width: stretch::style::Dimension::Points(10.0), ..Default::default() },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 10.0);
        assert_eq!(layout.children[0].height, 100.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 10.0);
        assert_eq!(layout.children[1].height, 100.0);
        assert_eq!(layout.children[1].x, 10.0);
        assert_eq!(layout.children[1].y, 0.0);

        assert_eq!(layout.children[2].width, 10.0);
        assert_eq!(layout.children[2].height, 100.0);
        assert_eq!(layout.children[2].x, 20.0);
        assert_eq!(layout.children[2].y, 0.0);
    }

    #[test]
    fn child_min_max_width_flexing() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(120.0),
            height: stretch::style::Dimension::Points(50.0),
            children: vec![
                stretch::style::Node {
                    flex_grow: 1.0,
                    flex_shrink: 0.0,
                    flex_basis: stretch::style::Dimension::Points(0.0),
                    min_width: stretch::style::Dimension::Points(60.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 1.0,
                    flex_shrink: 0.0,
                    flex_basis: stretch::style::Dimension::Percent(0.5),
                    max_width: stretch::style::Dimension::Points(20.0),
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 120.0);
        assert_eq!(layout.height, 50.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 100.0);
        assert_eq!(layout.children[0].height, 50.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 20.0);
        assert_eq!(layout.children[1].height, 50.0);
        assert_eq!(layout.children[1].x, 100.0);
        assert_eq!(layout.children[1].y, 0.0);
    }

    #[test]
    fn absolute_layout_in_wrap_reverse_row_container_flex_end() {
        let layout = stretch::compute(&stretch::style::Node {
            flexWrap: stretch::style::FlexWrap::WrapReverse,
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![stretch::style::Node {
                position: stretch::style::Position::Absolute,
                align_self: stretch::style::AlignSelf::FlexEnd,
                width: stretch::style::Dimension::Points(20.0),
                height: stretch::style::Dimension::Points(20.0),
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 20.0);
        assert_eq!(layout.children[0].height, 20.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);
    }

    #[test]
    fn margin_and_flex_column() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![stretch::style::Node {
                flex_grow: 1.0,
                margin: stretch::style::Edges {
                    top: stretch::style::Dimension::Points(10.0),
                    bottom: stretch::style::Dimension::Points(10.0),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 100.0);
        assert_eq!(layout.children[0].height, 80.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 10.0);
    }

    #[test]
    fn flex_basis_flex_shrink_row() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![
                stretch::style::Node { flex_basis: stretch::style::Dimension::Points(100.0), ..Default::default() },
                stretch::style::Node { flex_basis: stretch::style::Dimension::Points(50.0), ..Default::default() },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 67.0);
        assert_eq!(layout.children[0].height, 100.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 33.0);
        assert_eq!(layout.children[1].height, 100.0);
        assert_eq!(layout.children[1].x, 67.0);
        assert_eq!(layout.children[1].y, 0.0);
    }

    #[test]
    fn justify_content_overflow_min_max() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            justify_content: stretch::style::JustifyContent::Center,
            min_height: stretch::style::Dimension::Points(100.0),
            max_height: stretch::style::Dimension::Points(110.0),
            children: vec![
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(50.0),
                    height: stretch::style::Dimension::Points(50.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(50.0),
                    height: stretch::style::Dimension::Points(50.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(50.0),
                    height: stretch::style::Dimension::Points(50.0),
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 50.0);
        assert_eq!(layout.height, 110.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 50.0);
        assert_eq!(layout.children[0].height, 37.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 50.0);
        assert_eq!(layout.children[1].height, 36.0);
        assert_eq!(layout.children[1].x, 0.0);
        assert_eq!(layout.children[1].y, 37.0);

        assert_eq!(layout.children[2].width, 50.0);
        assert_eq!(layout.children[2].height, 37.0);
        assert_eq!(layout.children[2].x, 0.0);
        assert_eq!(layout.children[2].y, 73.0);
    }

    #[test]
    fn wrap_reverse_column_fixed_size() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            flexWrap: stretch::style::FlexWrap::WrapReverse,
            align_items: stretch::style::AlignItems::Center,
            width: stretch::style::Dimension::Points(200.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(30.0),
                    height: stretch::style::Dimension::Points(10.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(30.0),
                    height: stretch::style::Dimension::Points(20.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(30.0),
                    height: stretch::style::Dimension::Points(30.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(30.0),
                    height: stretch::style::Dimension::Points(40.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(30.0),
                    height: stretch::style::Dimension::Points(50.0),
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 200.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 30.0);
        assert_eq!(layout.children[0].height, 10.0);
        assert_eq!(layout.children[0].x, 135.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 30.0);
        assert_eq!(layout.children[1].height, 20.0);
        assert_eq!(layout.children[1].x, 135.0);
        assert_eq!(layout.children[1].y, 10.0);

        assert_eq!(layout.children[2].width, 30.0);
        assert_eq!(layout.children[2].height, 30.0);
        assert_eq!(layout.children[2].x, 135.0);
        assert_eq!(layout.children[2].y, 30.0);

        assert_eq!(layout.children[3].width, 30.0);
        assert_eq!(layout.children[3].height, 40.0);
        assert_eq!(layout.children[3].x, 135.0);
        assert_eq!(layout.children[3].y, 60.0);

        assert_eq!(layout.children[4].width, 30.0);
        assert_eq!(layout.children[4].height, 50.0);
        assert_eq!(layout.children[4].x, 35.0);
        assert_eq!(layout.children[4].y, 0.0);
    }

    #[test]
    fn max_width() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![stretch::style::Node {
                max_width: stretch::style::Dimension::Points(50.0),
                height: stretch::style::Dimension::Points(10.0),
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 50.0);
        assert_eq!(layout.children[0].height, 10.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);
    }

    #[test]
    fn justify_content_row_center() {
        let layout = stretch::compute(&stretch::style::Node {
            justify_content: stretch::style::JustifyContent::Center,
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![
                stretch::style::Node { width: stretch::style::Dimension::Points(10.0), ..Default::default() },
                stretch::style::Node { width: stretch::style::Dimension::Points(10.0), ..Default::default() },
                stretch::style::Node { width: stretch::style::Dimension::Points(10.0), ..Default::default() },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 10.0);
        assert_eq!(layout.children[0].height, 100.0);
        assert_eq!(layout.children[0].x, 35.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 10.0);
        assert_eq!(layout.children[1].height, 100.0);
        assert_eq!(layout.children[1].x, 45.0);
        assert_eq!(layout.children[1].y, 0.0);

        assert_eq!(layout.children[2].width, 10.0);
        assert_eq!(layout.children[2].height, 100.0);
        assert_eq!(layout.children[2].x, 55.0);
        assert_eq!(layout.children[2].y, 0.0);
    }

    #[test]
    fn flex_grow_root_minimized() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            width: stretch::style::Dimension::Points(100.0),
            min_height: stretch::style::Dimension::Points(100.0),
            max_height: stretch::style::Dimension::Points(500.0),
            children: vec![stretch::style::Node {
                flex_direction: stretch::style::FlexDirection::Column,
                flex_grow: 1.0,
                min_height: stretch::style::Dimension::Points(100.0),
                max_height: stretch::style::Dimension::Points(500.0),
                children: vec![
                    stretch::style::Node {
                        flex_grow: 1.0,
                        flex_basis: stretch::style::Dimension::Points(200.0),
                        ..Default::default()
                    },
                    stretch::style::Node { height: stretch::style::Dimension::Points(100.0), ..Default::default() },
                ],
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 300.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 100.0);
        assert_eq!(layout.children[0].height, 300.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[0].children[0].width, 100.0);
        assert_eq!(layout.children[0].children[0].height, 200.0);
        assert_eq!(layout.children[0].children[0].x, 0.0);
        assert_eq!(layout.children[0].children[0].y, 0.0);

        assert_eq!(layout.children[0].children[1].width, 100.0);
        assert_eq!(layout.children[0].children[1].height, 100.0);
        assert_eq!(layout.children[0].children[1].x, 0.0);
        assert_eq!(layout.children[0].children[1].y, 200.0);
    }

    #[test]
    fn container_with_unsized_child() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![stretch::style::Node { ..Default::default() }],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 0.0);
        assert_eq!(layout.children[0].height, 100.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);
    }

    #[test]
    fn flex_grow_within_constrained_min_row() {
        let layout = stretch::compute(&stretch::style::Node {
            min_width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![
                stretch::style::Node { flex_grow: 1.0, ..Default::default() },
                stretch::style::Node { width: stretch::style::Dimension::Points(50.0), ..Default::default() },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 50.0);
        assert_eq!(layout.children[0].height, 100.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 50.0);
        assert_eq!(layout.children[1].height, 100.0);
        assert_eq!(layout.children[1].x, 50.0);
        assert_eq!(layout.children[1].y, 0.0);
    }

    #[test]
    fn absolute_layout_width_height_start_top_end_bottom() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![stretch::style::Node {
                position: stretch::style::Position::Absolute,
                width: stretch::style::Dimension::Points(10.0),
                height: stretch::style::Dimension::Points(10.0),
                start: stretch::style::Dimension::Points(10.0),
                end: stretch::style::Dimension::Points(10.0),
                top: stretch::style::Dimension::Points(10.0),
                bottom: stretch::style::Dimension::Points(10.0),
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 10.0);
        assert_eq!(layout.children[0].height, 10.0);
        assert_eq!(layout.children[0].x, 10.0);
        assert_eq!(layout.children[0].y, 10.0);
    }

    #[test]
    fn overflow_main_axis() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![stretch::style::Node {
                flex_shrink: 0.0,
                width: stretch::style::Dimension::Points(200.0),
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 200.0);
        assert_eq!(layout.children[0].height, 100.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);
    }

    #[test]
    fn flex_grow_to_min() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            width: stretch::style::Dimension::Points(100.0),
            min_height: stretch::style::Dimension::Points(100.0),
            max_height: stretch::style::Dimension::Points(500.0),
            children: vec![
                stretch::style::Node { flex_grow: 1.0, flex_shrink: 1.0, ..Default::default() },
                stretch::style::Node { height: stretch::style::Dimension::Points(50.0), ..Default::default() },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 100.0);
        assert_eq!(layout.children[0].height, 50.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 100.0);
        assert_eq!(layout.children[1].height, 50.0);
        assert_eq!(layout.children[1].x, 0.0);
        assert_eq!(layout.children[1].y, 50.0);
    }

    #[test]
    fn absolute_layout_align_items_and_justify_content_center_and_top_position() {
        let layout = stretch::compute(&stretch::style::Node {
            align_items: stretch::style::AlignItems::Center,
            justify_content: stretch::style::JustifyContent::Center,
            width: stretch::style::Dimension::Points(110.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![stretch::style::Node {
                position: stretch::style::Position::Absolute,
                width: stretch::style::Dimension::Points(60.0),
                height: stretch::style::Dimension::Points(40.0),
                top: stretch::style::Dimension::Points(10.0),
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 110.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 60.0);
        assert_eq!(layout.children[0].height, 40.0);
        assert_eq!(layout.children[0].x, 25.0);
        assert_eq!(layout.children[0].y, 10.0);
    }

    #[test]
    fn min_height() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![
                stretch::style::Node {
                    flex_grow: 1.0,
                    min_height: stretch::style::Dimension::Points(60.0),
                    ..Default::default()
                },
                stretch::style::Node { flex_grow: 1.0, ..Default::default() },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 100.0);
        assert_eq!(layout.children[0].height, 80.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 100.0);
        assert_eq!(layout.children[1].height, 20.0);
        assert_eq!(layout.children[1].x, 0.0);
        assert_eq!(layout.children[1].y, 80.0);
    }

    #[test]
    fn justify_content_column_flex_start() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![
                stretch::style::Node { height: stretch::style::Dimension::Points(10.0), ..Default::default() },
                stretch::style::Node { height: stretch::style::Dimension::Points(10.0), ..Default::default() },
                stretch::style::Node { height: stretch::style::Dimension::Points(10.0), ..Default::default() },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 100.0);
        assert_eq!(layout.children[0].height, 10.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 100.0);
        assert_eq!(layout.children[1].height, 10.0);
        assert_eq!(layout.children[1].x, 0.0);
        assert_eq!(layout.children[1].y, 10.0);

        assert_eq!(layout.children[2].width, 100.0);
        assert_eq!(layout.children[2].height, 10.0);
        assert_eq!(layout.children[2].x, 0.0);
        assert_eq!(layout.children[2].y, 20.0);
    }

    #[test]
    fn absolute_layout_in_wrap_reverse_column_container() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            flexWrap: stretch::style::FlexWrap::WrapReverse,
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![stretch::style::Node {
                position: stretch::style::Position::Absolute,
                width: stretch::style::Dimension::Points(20.0),
                height: stretch::style::Dimension::Points(20.0),
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 20.0);
        assert_eq!(layout.children[0].height, 20.0);
        assert_eq!(layout.children[0].x, 80.0);
        assert_eq!(layout.children[0].y, 0.0);
    }

    #[test]
    fn margin_should_not_be_part_of_max_width() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(250.0),
            height: stretch::style::Dimension::Points(250.0),
            children: vec![stretch::style::Node {
                width: stretch::style::Dimension::Points(100.0),
                max_width: stretch::style::Dimension::Points(100.0),
                height: stretch::style::Dimension::Points(100.0),
                margin: stretch::style::Edges { start: stretch::style::Dimension::Points(20.0), ..Default::default() },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 250.0);
        assert_eq!(layout.height, 250.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 100.0);
        assert_eq!(layout.children[0].height, 100.0);
        assert_eq!(layout.children[0].x, 20.0);
        assert_eq!(layout.children[0].y, 0.0);
    }

    #[test]
    fn border_flex_child() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            border: stretch::style::Edges {
                start: stretch::style::Dimension::Points(10.0),
                end: stretch::style::Dimension::Points(10.0),
                top: stretch::style::Dimension::Points(10.0),
                bottom: stretch::style::Dimension::Points(10.0),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                flex_grow: 1.0,
                width: stretch::style::Dimension::Points(10.0),
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 80.0);
        assert_eq!(layout.children[0].height, 80.0);
        assert_eq!(layout.children[0].x, 10.0);
        assert_eq!(layout.children[0].y, 10.0);
    }

    #[test]
    fn wrap_reverse_row() {
        let layout = stretch::compute(&stretch::style::Node {
            flexWrap: stretch::style::FlexWrap::WrapReverse,
            width: stretch::style::Dimension::Points(100.0),
            children: vec![
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(30.0),
                    height: stretch::style::Dimension::Points(30.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(30.0),
                    height: stretch::style::Dimension::Points(30.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(30.0),
                    height: stretch::style::Dimension::Points(30.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(30.0),
                    height: stretch::style::Dimension::Points(30.0),
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 60.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 30.0);
        assert_eq!(layout.children[0].height, 30.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 30.0);

        assert_eq!(layout.children[1].width, 30.0);
        assert_eq!(layout.children[1].height, 30.0);
        assert_eq!(layout.children[1].x, 30.0);
        assert_eq!(layout.children[1].y, 30.0);

        assert_eq!(layout.children[2].width, 30.0);
        assert_eq!(layout.children[2].height, 30.0);
        assert_eq!(layout.children[2].x, 60.0);
        assert_eq!(layout.children[2].y, 30.0);

        assert_eq!(layout.children[3].width, 30.0);
        assert_eq!(layout.children[3].height, 30.0);
        assert_eq!(layout.children[3].x, 0.0);
        assert_eq!(layout.children[3].y, 0.0);
    }

    #[test]
    fn margin_right() {
        let layout = stretch::compute(&stretch::style::Node {
            justify_content: stretch::style::JustifyContent::FlexEnd,
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![stretch::style::Node {
                width: stretch::style::Dimension::Points(10.0),
                margin: stretch::style::Edges { end: stretch::style::Dimension::Points(10.0), ..Default::default() },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 10.0);
        assert_eq!(layout.children[0].height, 100.0);
        assert_eq!(layout.children[0].x, 80.0);
        assert_eq!(layout.children[0].y, 0.0);
    }

    #[test]
    fn align_flex_start_with_shrinking_children_with_stretch() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(500.0),
            height: stretch::style::Dimension::Points(500.0),
            children: vec![stretch::style::Node {
                align_items: stretch::style::AlignItems::FlexStart,
                children: vec![stretch::style::Node {
                    flex_grow: 1.0,
                    flex_shrink: 1.0,
                    children: vec![stretch::style::Node { flex_grow: 1.0, flex_shrink: 1.0, ..Default::default() }],
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 500.0);
        assert_eq!(layout.height, 500.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 0.0);
        assert_eq!(layout.children[0].height, 500.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[0].children[0].width, 0.0);
        assert_eq!(layout.children[0].children[0].height, 0.0);
        assert_eq!(layout.children[0].children[0].x, 0.0);
        assert_eq!(layout.children[0].children[0].y, 0.0);

        assert_eq!(layout.children[0].children[0].children[0].width, 0.0);
        assert_eq!(layout.children[0].children[0].children[0].height, 0.0);
        assert_eq!(layout.children[0].children[0].children[0].x, 0.0);
        assert_eq!(layout.children[0].children[0].children[0].y, 0.0);
    }

    #[test]
    fn align_items_stretch() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![stretch::style::Node {
                width: stretch::style::Dimension::Points(10.0),
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 10.0);
        assert_eq!(layout.children[0].height, 100.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);
    }

    #[test]
    fn parent_wrap_child_size_overflowing_parent() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![stretch::style::Node {
                width: stretch::style::Dimension::Points(100.0),
                children: vec![stretch::style::Node {
                    width: stretch::style::Dimension::Points(100.0),
                    height: stretch::style::Dimension::Points(200.0),
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 100.0);
        assert_eq!(layout.children[0].height, 100.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[0].children[0].width, 100.0);
        assert_eq!(layout.children[0].children[0].height, 200.0);
        assert_eq!(layout.children[0].children[0].x, 0.0);
        assert_eq!(layout.children[0].children[0].y, 0.0);
    }

    #[test]
    fn padding_stretch_child() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            padding: stretch::style::Edges {
                start: stretch::style::Dimension::Points(10.0),
                end: stretch::style::Dimension::Points(10.0),
                top: stretch::style::Dimension::Points(10.0),
                bottom: stretch::style::Dimension::Points(10.0),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                height: stretch::style::Dimension::Points(10.0),
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 0.0);
        assert_eq!(layout.children[0].height, 10.0);
        assert_eq!(layout.children[0].x, 10.0);
        assert_eq!(layout.children[0].y, 10.0);
    }

    #[test]
    fn wrap_nodes_with_content_sizing_overflowing_margin() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            width: stretch::style::Dimension::Points(500.0),
            height: stretch::style::Dimension::Points(500.0),
            children: vec![stretch::style::Node {
                flexWrap: stretch::style::FlexWrap::Wrap,
                width: stretch::style::Dimension::Points(85.0),
                children: vec![
                    stretch::style::Node {
                        flex_direction: stretch::style::FlexDirection::Column,
                        children: vec![stretch::style::Node {
                            width: stretch::style::Dimension::Points(40.0),
                            height: stretch::style::Dimension::Points(40.0),
                            ..Default::default()
                        }],
                        ..Default::default()
                    },
                    stretch::style::Node {
                        flex_direction: stretch::style::FlexDirection::Column,
                        margin: stretch::style::Edges {
                            end: stretch::style::Dimension::Points(10.0),
                            ..Default::default()
                        },
                        children: vec![stretch::style::Node {
                            width: stretch::style::Dimension::Points(40.0),
                            height: stretch::style::Dimension::Points(40.0),
                            ..Default::default()
                        }],
                        ..Default::default()
                    },
                ],
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 500.0);
        assert_eq!(layout.height, 500.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 85.0);
        assert_eq!(layout.children[0].height, 80.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[0].children[0].width, 40.0);
        assert_eq!(layout.children[0].children[0].height, 40.0);
        assert_eq!(layout.children[0].children[0].x, 0.0);
        assert_eq!(layout.children[0].children[0].y, 0.0);

        assert_eq!(layout.children[0].children[0].children[0].width, 40.0);
        assert_eq!(layout.children[0].children[0].children[0].height, 40.0);
        assert_eq!(layout.children[0].children[0].children[0].x, 0.0);
        assert_eq!(layout.children[0].children[0].children[0].y, 0.0);

        assert_eq!(layout.children[0].children[1].width, 40.0);
        assert_eq!(layout.children[0].children[1].height, 40.0);
        assert_eq!(layout.children[0].children[1].x, 0.0);
        assert_eq!(layout.children[0].children[1].y, 40.0);

        assert_eq!(layout.children[0].children[1].children[0].width, 40.0);
        assert_eq!(layout.children[0].children[1].children[0].height, 40.0);
        assert_eq!(layout.children[0].children[1].children[0].x, 0.0);
        assert_eq!(layout.children[0].children[1].children[0].y, 0.0);
    }

    #[test]
    fn flex_shrink_flex_grow_row() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(500.0),
            height: stretch::style::Dimension::Points(500.0),
            children: vec![
                stretch::style::Node {
                    flex_grow: 0.0,
                    flex_shrink: 1.0,
                    width: stretch::style::Dimension::Points(500.0),
                    height: stretch::style::Dimension::Points(100.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 0.0,
                    flex_shrink: 1.0,
                    width: stretch::style::Dimension::Points(500.0),
                    height: stretch::style::Dimension::Points(100.0),
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 500.0);
        assert_eq!(layout.height, 500.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 250.0);
        assert_eq!(layout.children[0].height, 100.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 250.0);
        assert_eq!(layout.children[1].height, 100.0);
        assert_eq!(layout.children[1].x, 250.0);
        assert_eq!(layout.children[1].y, 0.0);
    }

    #[test]
    fn flex_basis_flex_shrink_column() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![
                stretch::style::Node { flex_basis: stretch::style::Dimension::Points(100.0), ..Default::default() },
                stretch::style::Node { flex_basis: stretch::style::Dimension::Points(50.0), ..Default::default() },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 100.0);
        assert_eq!(layout.children[0].height, 67.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 100.0);
        assert_eq!(layout.children[1].height, 33.0);
        assert_eq!(layout.children[1].x, 0.0);
        assert_eq!(layout.children[1].y, 67.0);
    }

    #[test]
    fn align_items_min_max() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            align_items: stretch::style::AlignItems::Center,
            min_width: stretch::style::Dimension::Points(100.0),
            max_width: stretch::style::Dimension::Points(200.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![stretch::style::Node {
                width: stretch::style::Dimension::Points(60.0),
                height: stretch::style::Dimension::Points(60.0),
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 60.0);
        assert_eq!(layout.children[0].height, 60.0);
        assert_eq!(layout.children[0].x, 20.0);
        assert_eq!(layout.children[0].y, 0.0);
    }

    #[test]
    fn justify_content_row_min_width_and_margin() {
        let layout = stretch::compute(&stretch::style::Node {
            justify_content: stretch::style::JustifyContent::Center,
            min_width: stretch::style::Dimension::Points(50.0),
            margin: stretch::style::Edges { start: stretch::style::Dimension::Points(100.0), ..Default::default() },
            children: vec![stretch::style::Node {
                width: stretch::style::Dimension::Points(20.0),
                height: stretch::style::Dimension::Points(20.0),
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 50.0);
        assert_eq!(layout.height, 20.0);
        assert_eq!(layout.x, 100.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 20.0);
        assert_eq!(layout.children[0].height, 20.0);
        assert_eq!(layout.children[0].x, 15.0);
        assert_eq!(layout.children[0].y, 0.0);
    }

    #[test]
    fn align_self_flex_start() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![stretch::style::Node {
                align_self: stretch::style::AlignSelf::FlexStart,
                width: stretch::style::Dimension::Points(10.0),
                height: stretch::style::Dimension::Points(10.0),
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 10.0);
        assert_eq!(layout.children[0].height, 10.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);
    }

    #[test]
    fn percentage_position_left_top() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(400.0),
            height: stretch::style::Dimension::Points(400.0),
            children: vec![stretch::style::Node {
                width: stretch::style::Dimension::Percent(0.4),
                height: stretch::style::Dimension::Percent(0.6),
                start: stretch::style::Dimension::Percent(0.1),
                top: stretch::style::Dimension::Percent(0.2),
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 400.0);
        assert_eq!(layout.height, 400.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 180.0);
        assert_eq!(layout.children[0].height, 220.0);
        assert_eq!(layout.children[0].x, 40.0);
        assert_eq!(layout.children[0].y, 80.0);
    }

    #[test]
    fn border_stretch_child() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            border: stretch::style::Edges {
                start: stretch::style::Dimension::Points(10.0),
                end: stretch::style::Dimension::Points(10.0),
                top: stretch::style::Dimension::Points(10.0),
                bottom: stretch::style::Dimension::Points(10.0),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                width: stretch::style::Dimension::Points(10.0),
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 10.0);
        assert_eq!(layout.children[0].height, 80.0);
        assert_eq!(layout.children[0].x, 10.0);
        assert_eq!(layout.children[0].y, 10.0);
    }

    #[test]
    fn rounding_fractial_input_2() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(113.6),
            children: vec![
                stretch::style::Node {
                    flex_grow: 1.0,
                    flex_basis: stretch::style::Dimension::Points(50.0),
                    height: stretch::style::Dimension::Points(20.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 1.0,
                    height: stretch::style::Dimension::Points(10.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 1.0,
                    height: stretch::style::Dimension::Points(10.0),
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 114.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 100.0);
        assert_eq!(layout.children[0].height, 65.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 100.0);
        assert_eq!(layout.children[1].height, 24.0);
        assert_eq!(layout.children[1].x, 0.0);
        assert_eq!(layout.children[1].y, 65.0);

        assert_eq!(layout.children[2].width, 100.0);
        assert_eq!(layout.children[2].height, 25.0);
        assert_eq!(layout.children[2].x, 0.0);
        assert_eq!(layout.children[2].y, 89.0);
    }

    #[test]
    fn percentage_flex_basis_main_max_width() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(200.0),
            height: stretch::style::Dimension::Points(200.0),
            children: vec![
                stretch::style::Node {
                    flex_grow: 1.0,
                    flex_basis: stretch::style::Dimension::Percent(0.2),
                    max_width: stretch::style::Dimension::Percent(0.6),
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 4.0,
                    flex_basis: stretch::style::Dimension::Percent(0.1),
                    max_width: stretch::style::Dimension::Percent(0.2),
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 200.0);
        assert_eq!(layout.height, 200.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 120.0);
        assert_eq!(layout.children[0].height, 200.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 40.0);
        assert_eq!(layout.children[1].height, 200.0);
        assert_eq!(layout.children[1].x, 120.0);
        assert_eq!(layout.children[1].y, 0.0);
    }

    #[test]
    fn margin_auto_top_and_bottom_strech() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            width: stretch::style::Dimension::Points(200.0),
            height: stretch::style::Dimension::Points(200.0),
            children: vec![
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(50.0),
                    height: stretch::style::Dimension::Points(50.0),
                    margin: stretch::style::Edges {
                        top: stretch::style::Dimension::Auto,
                        bottom: stretch::style::Dimension::Auto,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(50.0),
                    height: stretch::style::Dimension::Points(50.0),
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 200.0);
        assert_eq!(layout.height, 200.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 50.0);
        assert_eq!(layout.children[0].height, 50.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 50.0);

        assert_eq!(layout.children[1].width, 50.0);
        assert_eq!(layout.children[1].height, 50.0);
        assert_eq!(layout.children[1].x, 0.0);
        assert_eq!(layout.children[1].y, 150.0);
    }

    #[test]
    fn percentage_multiple_nested_with_padding_margin_and_percentage_values() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            width: stretch::style::Dimension::Points(200.0),
            height: stretch::style::Dimension::Points(200.0),
            children: vec![
                stretch::style::Node {
                    flex_direction: stretch::style::FlexDirection::Column,
                    flex_grow: 1.0,
                    flex_basis: stretch::style::Dimension::Percent(0.1),
                    min_width: stretch::style::Dimension::Percent(0.6),
                    margin: stretch::style::Edges {
                        start: stretch::style::Dimension::Points(5.0),
                        end: stretch::style::Dimension::Points(5.0),
                        top: stretch::style::Dimension::Points(5.0),
                        bottom: stretch::style::Dimension::Points(5.0),
                        ..Default::default()
                    },
                    padding: stretch::style::Edges {
                        start: stretch::style::Dimension::Points(3.0),
                        end: stretch::style::Dimension::Points(3.0),
                        top: stretch::style::Dimension::Points(3.0),
                        bottom: stretch::style::Dimension::Points(3.0),
                        ..Default::default()
                    },
                    children: vec![stretch::style::Node {
                        flex_direction: stretch::style::FlexDirection::Column,
                        width: stretch::style::Dimension::Percent(0.5),
                        margin: stretch::style::Edges {
                            start: stretch::style::Dimension::Points(5.0),
                            end: stretch::style::Dimension::Points(5.0),
                            top: stretch::style::Dimension::Points(5.0),
                            bottom: stretch::style::Dimension::Points(5.0),
                            ..Default::default()
                        },
                        padding: stretch::style::Edges {
                            start: stretch::style::Dimension::Percent(0.0),
                            end: stretch::style::Dimension::Percent(0.0),
                            top: stretch::style::Dimension::Percent(0.0),
                            bottom: stretch::style::Dimension::Percent(0.0),
                            ..Default::default()
                        },
                        children: vec![stretch::style::Node {
                            width: stretch::style::Dimension::Percent(0.4),
                            margin: stretch::style::Edges {
                                start: stretch::style::Dimension::Percent(0.1),
                                end: stretch::style::Dimension::Percent(0.1),
                                top: stretch::style::Dimension::Percent(0.1),
                                bottom: stretch::style::Dimension::Percent(0.1),
                                ..Default::default()
                            },
                            padding: stretch::style::Edges {
                                start: stretch::style::Dimension::Points(3.0),
                                end: stretch::style::Dimension::Points(3.0),
                                top: stretch::style::Dimension::Points(3.0),
                                bottom: stretch::style::Dimension::Points(3.0),
                                ..Default::default()
                            },
                            ..Default::default()
                        }],
                        ..Default::default()
                    }],
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 4.0,
                    flex_basis: stretch::style::Dimension::Percent(0.2),
                    min_width: stretch::style::Dimension::Percent(0.2),
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 200.0);
        assert_eq!(layout.height, 200.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 190.0);
        assert_eq!(layout.children[0].height, 48.0);
        assert_eq!(layout.children[0].x, 5.0);
        assert_eq!(layout.children[0].y, 5.0);

        assert_eq!(layout.children[0].children[0].width, 92.0);
        assert_eq!(layout.children[0].children[0].height, 25.0);
        assert_eq!(layout.children[0].children[0].x, 8.0);
        assert_eq!(layout.children[0].children[0].y, 8.0);

        assert_eq!(layout.children[0].children[0].children[0].width, 36.0);
        assert_eq!(layout.children[0].children[0].children[0].height, 6.0);
        assert_eq!(layout.children[0].children[0].children[0].x, 10.0);
        assert_eq!(layout.children[0].children[0].children[0].y, 10.0);

        assert_eq!(layout.children[1].width, 200.0);
        assert_eq!(layout.children[1].height, 142.0);
        assert_eq!(layout.children[1].x, 0.0);
        assert_eq!(layout.children[1].y, 58.0);
    }

    #[test]
    fn flex_grow_within_constrained_min_column() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            min_height: stretch::style::Dimension::Points(100.0),
            children: vec![
                stretch::style::Node { flex_grow: 1.0, ..Default::default() },
                stretch::style::Node { height: stretch::style::Dimension::Points(50.0), ..Default::default() },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 0.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 0.0);
        assert_eq!(layout.children[0].height, 50.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 0.0);
        assert_eq!(layout.children[1].height, 50.0);
        assert_eq!(layout.children[1].x, 0.0);
        assert_eq!(layout.children[1].y, 50.0);
    }

    #[test]
    fn flex_grow_in_at_most_container() {
        let layout = stretch::compute(&stretch::style::Node {
            align_items: stretch::style::AlignItems::FlexStart,
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![stretch::style::Node {
                children: vec![stretch::style::Node {
                    flex_grow: 1.0,
                    flex_basis: stretch::style::Dimension::Points(0.0),
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 0.0);
        assert_eq!(layout.children[0].height, 0.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[0].children[0].width, 0.0);
        assert_eq!(layout.children[0].children[0].height, 0.0);
        assert_eq!(layout.children[0].children[0].x, 0.0);
        assert_eq!(layout.children[0].children[0].y, 0.0);
    }

    #[test]
    fn margin_auto_left() {
        let layout = stretch::compute(&stretch::style::Node {
            align_items: stretch::style::AlignItems::Center,
            width: stretch::style::Dimension::Points(200.0),
            height: stretch::style::Dimension::Points(200.0),
            children: vec![
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(50.0),
                    height: stretch::style::Dimension::Points(50.0),
                    margin: stretch::style::Edges { start: stretch::style::Dimension::Auto, ..Default::default() },
                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(50.0),
                    height: stretch::style::Dimension::Points(50.0),
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 200.0);
        assert_eq!(layout.height, 200.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 50.0);
        assert_eq!(layout.children[0].height, 50.0);
        assert_eq!(layout.children[0].x, 100.0);
        assert_eq!(layout.children[0].y, 75.0);

        assert_eq!(layout.children[1].width, 50.0);
        assert_eq!(layout.children[1].height, 50.0);
        assert_eq!(layout.children[1].x, 150.0);
        assert_eq!(layout.children[1].y, 75.0);
    }

    #[test]
    fn percentage_flex_basis_cross() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            width: stretch::style::Dimension::Points(200.0),
            height: stretch::style::Dimension::Points(200.0),
            children: vec![
                stretch::style::Node {
                    flex_grow: 1.0,
                    flex_basis: stretch::style::Dimension::Percent(0.5),
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 1.0,
                    flex_basis: stretch::style::Dimension::Percent(0.2),
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 200.0);
        assert_eq!(layout.height, 200.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 200.0);
        assert_eq!(layout.children[0].height, 125.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 200.0);
        assert_eq!(layout.children[1].height, 75.0);
        assert_eq!(layout.children[1].x, 0.0);
        assert_eq!(layout.children[1].y, 125.0);
    }

    #[test]
    fn percentage_absolute_position() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            width: stretch::style::Dimension::Points(200.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![stretch::style::Node {
                position: stretch::style::Position::Absolute,
                width: stretch::style::Dimension::Points(10.0),
                height: stretch::style::Dimension::Points(10.0),
                start: stretch::style::Dimension::Percent(0.3),
                top: stretch::style::Dimension::Percent(0.1),
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 200.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 10.0);
        assert_eq!(layout.children[0].height, 10.0);
        assert_eq!(layout.children[0].x, 60.0);
        assert_eq!(layout.children[0].y, 10.0);
    }

    #[test]
    fn flex_basis_flex_grow_row() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![
                stretch::style::Node {
                    flex_grow: 1.0,
                    flex_basis: stretch::style::Dimension::Points(50.0),
                    ..Default::default()
                },
                stretch::style::Node { flex_grow: 1.0, ..Default::default() },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 75.0);
        assert_eq!(layout.children[0].height, 100.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 25.0);
        assert_eq!(layout.children[1].height, 100.0);
        assert_eq!(layout.children[1].x, 75.0);
        assert_eq!(layout.children[1].y, 0.0);
    }

    #[test]
    fn absolute_layout_in_wrap_reverse_column_container_flex_end() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            flexWrap: stretch::style::FlexWrap::WrapReverse,
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![stretch::style::Node {
                position: stretch::style::Position::Absolute,
                align_self: stretch::style::AlignSelf::FlexEnd,
                width: stretch::style::Dimension::Points(20.0),
                height: stretch::style::Dimension::Points(20.0),
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 20.0);
        assert_eq!(layout.children[0].height, 20.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);
    }

    #[test]
    fn justify_content_row_flex_start() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![
                stretch::style::Node { width: stretch::style::Dimension::Points(10.0), ..Default::default() },
                stretch::style::Node { width: stretch::style::Dimension::Points(10.0), ..Default::default() },
                stretch::style::Node { width: stretch::style::Dimension::Points(10.0), ..Default::default() },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 10.0);
        assert_eq!(layout.children[0].height, 100.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 10.0);
        assert_eq!(layout.children[1].height, 100.0);
        assert_eq!(layout.children[1].x, 10.0);
        assert_eq!(layout.children[1].y, 0.0);

        assert_eq!(layout.children[2].width, 10.0);
        assert_eq!(layout.children[2].height, 100.0);
        assert_eq!(layout.children[2].x, 20.0);
        assert_eq!(layout.children[2].y, 0.0);
    }

    #[test]
    fn margin_auto_left_right_child_bigger_than_parent() {
        let layout = stretch::compute(&stretch::style::Node {
            justify_content: stretch::style::JustifyContent::Center,
            width: stretch::style::Dimension::Points(52.0),
            height: stretch::style::Dimension::Points(52.0),
            children: vec![stretch::style::Node {
                width: stretch::style::Dimension::Points(72.0),
                height: stretch::style::Dimension::Points(72.0),
                margin: stretch::style::Edges {
                    start: stretch::style::Dimension::Auto,
                    end: stretch::style::Dimension::Auto,
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 52.0);
        assert_eq!(layout.height, 52.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 52.0);
        assert_eq!(layout.children[0].height, 72.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);
    }

    #[test]
    fn margin_fix_left_auto_right_child_bigger_than_parent() {
        let layout = stretch::compute(&stretch::style::Node {
            justify_content: stretch::style::JustifyContent::Center,
            width: stretch::style::Dimension::Points(52.0),
            height: stretch::style::Dimension::Points(52.0),
            children: vec![stretch::style::Node {
                width: stretch::style::Dimension::Points(72.0),
                height: stretch::style::Dimension::Points(72.0),
                margin: stretch::style::Edges {
                    start: stretch::style::Dimension::Points(10.0),
                    end: stretch::style::Dimension::Auto,
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 52.0);
        assert_eq!(layout.height, 52.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 42.0);
        assert_eq!(layout.children[0].height, 72.0);
        assert_eq!(layout.children[0].x, 10.0);
        assert_eq!(layout.children[0].y, 0.0);
    }

    #[test]
    fn percentage_container_in_wrapping_container() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            align_items: stretch::style::AlignItems::Center,
            justify_content: stretch::style::JustifyContent::Center,
            width: stretch::style::Dimension::Points(200.0),
            height: stretch::style::Dimension::Points(200.0),
            children: vec![stretch::style::Node {
                flex_direction: stretch::style::FlexDirection::Column,
                children: vec![stretch::style::Node {
                    justify_content: stretch::style::JustifyContent::Center,
                    width: stretch::style::Dimension::Percent(1.0),
                    children: vec![
                        stretch::style::Node {
                            width: stretch::style::Dimension::Points(50.0),
                            height: stretch::style::Dimension::Points(50.0),
                            ..Default::default()
                        },
                        stretch::style::Node {
                            width: stretch::style::Dimension::Points(50.0),
                            height: stretch::style::Dimension::Points(50.0),
                            ..Default::default()
                        },
                    ],
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 200.0);
        assert_eq!(layout.height, 200.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 100.0);
        assert_eq!(layout.children[0].height, 50.0);
        assert_eq!(layout.children[0].x, 50.0);
        assert_eq!(layout.children[0].y, 75.0);

        assert_eq!(layout.children[0].children[0].width, 100.0);
        assert_eq!(layout.children[0].children[0].height, 50.0);
        assert_eq!(layout.children[0].children[0].x, 0.0);
        assert_eq!(layout.children[0].children[0].y, 0.0);

        assert_eq!(layout.children[0].children[0].children[0].width, 50.0);
        assert_eq!(layout.children[0].children[0].children[0].height, 50.0);
        assert_eq!(layout.children[0].children[0].children[0].x, 0.0);
        assert_eq!(layout.children[0].children[0].children[0].y, 0.0);

        assert_eq!(layout.children[0].children[0].children[1].width, 50.0);
        assert_eq!(layout.children[0].children[0].children[1].height, 50.0);
        assert_eq!(layout.children[0].children[0].children[1].x, 50.0);
        assert_eq!(layout.children[0].children[0].children[1].y, 0.0);
    }

    #[test]
    fn margin_auto_left_stretching_child() {
        let layout = stretch::compute(&stretch::style::Node {
            align_items: stretch::style::AlignItems::Center,
            width: stretch::style::Dimension::Points(200.0),
            height: stretch::style::Dimension::Points(200.0),
            children: vec![
                stretch::style::Node {
                    flex_grow: 1.0,
                    flex_shrink: 1.0,
                    flex_basis: stretch::style::Dimension::Percent(0.0),
                    margin: stretch::style::Edges { start: stretch::style::Dimension::Auto, ..Default::default() },
                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(50.0),
                    height: stretch::style::Dimension::Points(50.0),
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 200.0);
        assert_eq!(layout.height, 200.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 150.0);
        assert_eq!(layout.children[0].height, 0.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 100.0);

        assert_eq!(layout.children[1].width, 50.0);
        assert_eq!(layout.children[1].height, 50.0);
        assert_eq!(layout.children[1].x, 150.0);
        assert_eq!(layout.children[1].y, 75.0);
    }

    #[test]
    fn percentage_flex_basis_cross_max_width() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            width: stretch::style::Dimension::Points(200.0),
            height: stretch::style::Dimension::Points(200.0),
            children: vec![
                stretch::style::Node {
                    flex_grow: 1.0,
                    flex_basis: stretch::style::Dimension::Percent(0.1),
                    max_width: stretch::style::Dimension::Percent(0.6),
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 4.0,
                    flex_basis: stretch::style::Dimension::Percent(0.2),
                    max_width: stretch::style::Dimension::Percent(0.2),
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 200.0);
        assert_eq!(layout.height, 200.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 120.0);
        assert_eq!(layout.children[0].height, 50.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 40.0);
        assert_eq!(layout.children[1].height, 150.0);
        assert_eq!(layout.children[1].x, 0.0);
        assert_eq!(layout.children[1].y, 50.0);
    }

    #[test]
    fn align_items_flex_end_child_without_margin_bigger_than_parent() {
        let layout = stretch::compute(&stretch::style::Node {
            align_items: stretch::style::AlignItems::Center,
            justify_content: stretch::style::JustifyContent::Center,
            width: stretch::style::Dimension::Points(52.0),
            height: stretch::style::Dimension::Points(52.0),
            children: vec![stretch::style::Node {
                align_items: stretch::style::AlignItems::FlexEnd,
                children: vec![stretch::style::Node {
                    width: stretch::style::Dimension::Points(72.0),
                    height: stretch::style::Dimension::Points(72.0),
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 52.0);
        assert_eq!(layout.height, 52.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 72.0);
        assert_eq!(layout.children[0].height, 72.0);
        assert_eq!(layout.children[0].x, -10.0);
        assert_eq!(layout.children[0].y, -10.0);

        assert_eq!(layout.children[0].children[0].width, 72.0);
        assert_eq!(layout.children[0].children[0].height, 72.0);
        assert_eq!(layout.children[0].children[0].x, 0.0);
        assert_eq!(layout.children[0].children[0].y, 0.0);
    }

    #[test]
    fn size_defined_by_child_with_padding() {
        let layout = stretch::compute(&stretch::style::Node {
            padding: stretch::style::Edges {
                start: stretch::style::Dimension::Points(10.0),
                end: stretch::style::Dimension::Points(10.0),
                top: stretch::style::Dimension::Points(10.0),
                bottom: stretch::style::Dimension::Points(10.0),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                width: stretch::style::Dimension::Points(10.0),
                height: stretch::style::Dimension::Points(10.0),
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 30.0);
        assert_eq!(layout.height, 30.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 10.0);
        assert_eq!(layout.children[0].height, 10.0);
        assert_eq!(layout.children[0].x, 10.0);
        assert_eq!(layout.children[0].y, 10.0);
    }

    #[test]
    fn absolute_layout_within_border() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            padding: stretch::style::Edges {
                start: stretch::style::Dimension::Points(10.0),
                end: stretch::style::Dimension::Points(10.0),
                top: stretch::style::Dimension::Points(10.0),
                bottom: stretch::style::Dimension::Points(10.0),
                ..Default::default()
            },
            border: stretch::style::Edges {
                start: stretch::style::Dimension::Points(10.0),
                end: stretch::style::Dimension::Points(10.0),
                top: stretch::style::Dimension::Points(10.0),
                bottom: stretch::style::Dimension::Points(10.0),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    position: stretch::style::Position::Absolute,
                    width: stretch::style::Dimension::Points(50.0),
                    height: stretch::style::Dimension::Points(50.0),
                    start: stretch::style::Dimension::Points(0.0),
                    top: stretch::style::Dimension::Points(0.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    position: stretch::style::Position::Absolute,
                    width: stretch::style::Dimension::Points(50.0),
                    height: stretch::style::Dimension::Points(50.0),
                    end: stretch::style::Dimension::Points(0.0),
                    bottom: stretch::style::Dimension::Points(0.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    position: stretch::style::Position::Absolute,
                    width: stretch::style::Dimension::Points(50.0),
                    height: stretch::style::Dimension::Points(50.0),
                    margin: stretch::style::Edges {
                        start: stretch::style::Dimension::Points(10.0),
                        end: stretch::style::Dimension::Points(10.0),
                        top: stretch::style::Dimension::Points(10.0),
                        bottom: stretch::style::Dimension::Points(10.0),
                        ..Default::default()
                    },
                    start: stretch::style::Dimension::Points(0.0),
                    top: stretch::style::Dimension::Points(0.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    position: stretch::style::Position::Absolute,
                    width: stretch::style::Dimension::Points(50.0),
                    height: stretch::style::Dimension::Points(50.0),
                    margin: stretch::style::Edges {
                        start: stretch::style::Dimension::Points(10.0),
                        end: stretch::style::Dimension::Points(10.0),
                        top: stretch::style::Dimension::Points(10.0),
                        bottom: stretch::style::Dimension::Points(10.0),
                        ..Default::default()
                    },
                    end: stretch::style::Dimension::Points(0.0),
                    bottom: stretch::style::Dimension::Points(0.0),
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 50.0);
        assert_eq!(layout.children[0].height, 50.0);
        assert_eq!(layout.children[0].x, 10.0);
        assert_eq!(layout.children[0].y, 10.0);

        assert_eq!(layout.children[1].width, 50.0);
        assert_eq!(layout.children[1].height, 50.0);
        assert_eq!(layout.children[1].x, 40.0);
        assert_eq!(layout.children[1].y, 40.0);

        assert_eq!(layout.children[2].width, 50.0);
        assert_eq!(layout.children[2].height, 50.0);
        assert_eq!(layout.children[2].x, 20.0);
        assert_eq!(layout.children[2].y, 20.0);

        assert_eq!(layout.children[3].width, 50.0);
        assert_eq!(layout.children[3].height, 50.0);
        assert_eq!(layout.children[3].x, 30.0);
        assert_eq!(layout.children[3].y, 30.0);
    }

    #[test]
    fn margin_auto_left_fix_right_child_bigger_than_parent() {
        let layout = stretch::compute(&stretch::style::Node {
            justify_content: stretch::style::JustifyContent::Center,
            width: stretch::style::Dimension::Points(52.0),
            height: stretch::style::Dimension::Points(52.0),
            children: vec![stretch::style::Node {
                width: stretch::style::Dimension::Points(72.0),
                height: stretch::style::Dimension::Points(72.0),
                margin: stretch::style::Edges {
                    start: stretch::style::Dimension::Auto,
                    end: stretch::style::Dimension::Points(10.0),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 52.0);
        assert_eq!(layout.height, 52.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 42.0);
        assert_eq!(layout.children[0].height, 72.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);
    }

    #[test]
    fn justify_content_row_flex_end() {
        let layout = stretch::compute(&stretch::style::Node {
            justify_content: stretch::style::JustifyContent::FlexEnd,
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![
                stretch::style::Node { width: stretch::style::Dimension::Points(10.0), ..Default::default() },
                stretch::style::Node { width: stretch::style::Dimension::Points(10.0), ..Default::default() },
                stretch::style::Node { width: stretch::style::Dimension::Points(10.0), ..Default::default() },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 10.0);
        assert_eq!(layout.children[0].height, 100.0);
        assert_eq!(layout.children[0].x, 70.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 10.0);
        assert_eq!(layout.children[1].height, 100.0);
        assert_eq!(layout.children[1].x, 80.0);
        assert_eq!(layout.children[1].y, 0.0);

        assert_eq!(layout.children[2].width, 10.0);
        assert_eq!(layout.children[2].height, 100.0);
        assert_eq!(layout.children[2].x, 90.0);
        assert_eq!(layout.children[2].y, 0.0);
    }

    #[test]
    fn margin_bottom() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            justify_content: stretch::style::JustifyContent::FlexEnd,
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![stretch::style::Node {
                height: stretch::style::Dimension::Points(10.0),
                margin: stretch::style::Edges { bottom: stretch::style::Dimension::Points(10.0), ..Default::default() },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 100.0);
        assert_eq!(layout.children[0].height, 10.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 80.0);
    }

    #[test]
    fn margin_should_not_be_part_of_max_height() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(250.0),
            height: stretch::style::Dimension::Points(250.0),
            children: vec![stretch::style::Node {
                width: stretch::style::Dimension::Points(100.0),
                height: stretch::style::Dimension::Points(100.0),
                max_height: stretch::style::Dimension::Points(100.0),
                margin: stretch::style::Edges { top: stretch::style::Dimension::Points(20.0), ..Default::default() },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 250.0);
        assert_eq!(layout.height, 250.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 100.0);
        assert_eq!(layout.children[0].height, 100.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 20.0);
    }

    #[test]
    fn percentage_flex_basis_cross_min_height() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            width: stretch::style::Dimension::Points(200.0),
            height: stretch::style::Dimension::Points(200.0),
            children: vec![
                stretch::style::Node {
                    flex_grow: 1.0,
                    min_height: stretch::style::Dimension::Percent(0.6),
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 2.0,
                    min_height: stretch::style::Dimension::Percent(0.1),
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 200.0);
        assert_eq!(layout.height, 200.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 200.0);
        assert_eq!(layout.children[0].height, 140.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 200.0);
        assert_eq!(layout.children[1].height, 60.0);
        assert_eq!(layout.children[1].x, 0.0);
        assert_eq!(layout.children[1].y, 140.0);
    }

    #[test]
    fn nested_overflowing_child_in_constraint_parent() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![stretch::style::Node {
                width: stretch::style::Dimension::Points(100.0),
                height: stretch::style::Dimension::Points(100.0),
                children: vec![stretch::style::Node {
                    width: stretch::style::Dimension::Points(200.0),
                    height: stretch::style::Dimension::Points(200.0),
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 100.0);
        assert_eq!(layout.children[0].height, 100.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[0].children[0].width, 100.0);
        assert_eq!(layout.children[0].children[0].height, 200.0);
        assert_eq!(layout.children[0].children[0].x, 0.0);
        assert_eq!(layout.children[0].children[0].y, 0.0);
    }

    #[test]
    fn wrapped_row_within_align_items_center() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            align_items: stretch::style::AlignItems::Center,
            width: stretch::style::Dimension::Points(200.0),
            height: stretch::style::Dimension::Points(200.0),
            children: vec![stretch::style::Node {
                flexWrap: stretch::style::FlexWrap::Wrap,
                children: vec![
                    stretch::style::Node {
                        width: stretch::style::Dimension::Points(150.0),
                        height: stretch::style::Dimension::Points(80.0),
                        ..Default::default()
                    },
                    stretch::style::Node {
                        width: stretch::style::Dimension::Points(80.0),
                        height: stretch::style::Dimension::Points(80.0),
                        ..Default::default()
                    },
                ],
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 200.0);
        assert_eq!(layout.height, 200.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 200.0);
        assert_eq!(layout.children[0].height, 160.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[0].children[0].width, 150.0);
        assert_eq!(layout.children[0].children[0].height, 80.0);
        assert_eq!(layout.children[0].children[0].x, 0.0);
        assert_eq!(layout.children[0].children[0].y, 0.0);

        assert_eq!(layout.children[0].children[1].width, 80.0);
        assert_eq!(layout.children[0].children[1].height, 80.0);
        assert_eq!(layout.children[0].children[1].x, 0.0);
        assert_eq!(layout.children[0].children[1].y, 80.0);
    }

    #[test]
    fn justify_content_column_center() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            justify_content: stretch::style::JustifyContent::Center,
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![
                stretch::style::Node { height: stretch::style::Dimension::Points(10.0), ..Default::default() },
                stretch::style::Node { height: stretch::style::Dimension::Points(10.0), ..Default::default() },
                stretch::style::Node { height: stretch::style::Dimension::Points(10.0), ..Default::default() },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 100.0);
        assert_eq!(layout.children[0].height, 10.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 35.0);

        assert_eq!(layout.children[1].width, 100.0);
        assert_eq!(layout.children[1].height, 10.0);
        assert_eq!(layout.children[1].x, 0.0);
        assert_eq!(layout.children[1].y, 45.0);

        assert_eq!(layout.children[2].width, 100.0);
        assert_eq!(layout.children[2].height, 10.0);
        assert_eq!(layout.children[2].x, 0.0);
        assert_eq!(layout.children[2].y, 55.0);
    }

    #[test]
    fn align_items_center_child_with_margin_bigger_than_parent() {
        let layout = stretch::compute(&stretch::style::Node {
            align_items: stretch::style::AlignItems::Center,
            justify_content: stretch::style::JustifyContent::Center,
            width: stretch::style::Dimension::Points(52.0),
            height: stretch::style::Dimension::Points(52.0),
            children: vec![stretch::style::Node {
                align_items: stretch::style::AlignItems::Center,
                children: vec![stretch::style::Node {
                    width: stretch::style::Dimension::Points(52.0),
                    height: stretch::style::Dimension::Points(52.0),
                    margin: stretch::style::Edges {
                        start: stretch::style::Dimension::Points(10.0),
                        end: stretch::style::Dimension::Points(10.0),
                        ..Default::default()
                    },
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 52.0);
        assert_eq!(layout.height, 52.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 72.0);
        assert_eq!(layout.children[0].height, 52.0);
        assert_eq!(layout.children[0].x, -10.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[0].children[0].width, 52.0);
        assert_eq!(layout.children[0].children[0].height, 52.0);
        assert_eq!(layout.children[0].children[0].x, 10.0);
        assert_eq!(layout.children[0].children[0].y, 0.0);
    }

    #[test]
    fn percentage_width_height() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(200.0),
            height: stretch::style::Dimension::Points(200.0),
            children: vec![stretch::style::Node {
                width: stretch::style::Dimension::Percent(0.3),
                height: stretch::style::Dimension::Percent(0.3),
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 200.0);
        assert_eq!(layout.height, 200.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 60.0);
        assert_eq!(layout.children[0].height, 60.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);
    }

    #[test]
    fn margin_auto_mutiple_children_column() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            align_items: stretch::style::AlignItems::Center,
            width: stretch::style::Dimension::Points(200.0),
            height: stretch::style::Dimension::Points(200.0),
            children: vec![
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(50.0),
                    height: stretch::style::Dimension::Points(50.0),
                    margin: stretch::style::Edges { top: stretch::style::Dimension::Auto, ..Default::default() },
                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(50.0),
                    height: stretch::style::Dimension::Points(50.0),
                    margin: stretch::style::Edges { top: stretch::style::Dimension::Auto, ..Default::default() },
                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(50.0),
                    height: stretch::style::Dimension::Points(50.0),
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 200.0);
        assert_eq!(layout.height, 200.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 50.0);
        assert_eq!(layout.children[0].height, 50.0);
        assert_eq!(layout.children[0].x, 75.0);
        assert_eq!(layout.children[0].y, 25.0);

        assert_eq!(layout.children[1].width, 50.0);
        assert_eq!(layout.children[1].height, 50.0);
        assert_eq!(layout.children[1].x, 75.0);
        assert_eq!(layout.children[1].y, 100.0);

        assert_eq!(layout.children[2].width, 50.0);
        assert_eq!(layout.children[2].height, 50.0);
        assert_eq!(layout.children[2].x, 75.0);
        assert_eq!(layout.children[2].y, 150.0);
    }

    #[test]
    fn min_width_overrides_width_on_root() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(50.0),
            min_width: stretch::style::Dimension::Points(100.0),
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 0.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);
    }

    #[test]
    fn flex_wrap_wrap_to_child_height() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            children: vec![
                stretch::style::Node {
                    flexWrap: stretch::style::FlexWrap::Wrap,
                    align_items: stretch::style::AlignItems::FlexStart,
                    children: vec![stretch::style::Node {
                        flex_direction: stretch::style::FlexDirection::Column,
                        width: stretch::style::Dimension::Points(100.0),
                        children: vec![stretch::style::Node {
                            width: stretch::style::Dimension::Points(100.0),
                            height: stretch::style::Dimension::Points(100.0),
                            ..Default::default()
                        }],
                        ..Default::default()
                    }],
                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(100.0),
                    height: stretch::style::Dimension::Points(100.0),
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 200.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 100.0);
        assert_eq!(layout.children[0].height, 100.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[0].children[0].width, 100.0);
        assert_eq!(layout.children[0].children[0].height, 100.0);
        assert_eq!(layout.children[0].children[0].x, 0.0);
        assert_eq!(layout.children[0].children[0].y, 0.0);

        assert_eq!(layout.children[0].children[0].children[0].width, 100.0);
        assert_eq!(layout.children[0].children[0].children[0].height, 100.0);
        assert_eq!(layout.children[0].children[0].children[0].x, 0.0);
        assert_eq!(layout.children[0].children[0].children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 100.0);
        assert_eq!(layout.children[1].height, 100.0);
        assert_eq!(layout.children[1].x, 0.0);
        assert_eq!(layout.children[1].y, 100.0);
    }

    #[test]
    fn wrap_nodes_with_content_sizing_margin_cross() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            width: stretch::style::Dimension::Points(500.0),
            height: stretch::style::Dimension::Points(500.0),
            children: vec![stretch::style::Node {
                flexWrap: stretch::style::FlexWrap::Wrap,
                width: stretch::style::Dimension::Points(70.0),
                children: vec![
                    stretch::style::Node {
                        flex_direction: stretch::style::FlexDirection::Column,
                        children: vec![stretch::style::Node {
                            width: stretch::style::Dimension::Points(40.0),
                            height: stretch::style::Dimension::Points(40.0),
                            ..Default::default()
                        }],
                        ..Default::default()
                    },
                    stretch::style::Node {
                        flex_direction: stretch::style::FlexDirection::Column,
                        margin: stretch::style::Edges {
                            top: stretch::style::Dimension::Points(10.0),
                            ..Default::default()
                        },
                        children: vec![stretch::style::Node {
                            width: stretch::style::Dimension::Points(40.0),
                            height: stretch::style::Dimension::Points(40.0),
                            ..Default::default()
                        }],
                        ..Default::default()
                    },
                ],
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 500.0);
        assert_eq!(layout.height, 500.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 70.0);
        assert_eq!(layout.children[0].height, 90.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[0].children[0].width, 40.0);
        assert_eq!(layout.children[0].children[0].height, 40.0);
        assert_eq!(layout.children[0].children[0].x, 0.0);
        assert_eq!(layout.children[0].children[0].y, 0.0);

        assert_eq!(layout.children[0].children[0].children[0].width, 40.0);
        assert_eq!(layout.children[0].children[0].children[0].height, 40.0);
        assert_eq!(layout.children[0].children[0].children[0].x, 0.0);
        assert_eq!(layout.children[0].children[0].children[0].y, 0.0);

        assert_eq!(layout.children[0].children[1].width, 40.0);
        assert_eq!(layout.children[0].children[1].height, 40.0);
        assert_eq!(layout.children[0].children[1].x, 0.0);
        assert_eq!(layout.children[0].children[1].y, 50.0);

        assert_eq!(layout.children[0].children[1].children[0].width, 40.0);
        assert_eq!(layout.children[0].children[1].children[0].height, 40.0);
        assert_eq!(layout.children[0].children[1].children[0].x, 0.0);
        assert_eq!(layout.children[0].children[1].children[0].y, 0.0);
    }

    #[test]
    fn align_baseline() {
        let layout = stretch::compute(&stretch::style::Node {
            align_items: stretch::style::AlignItems::Baseline,
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(50.0),
                    height: stretch::style::Dimension::Points(50.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(50.0),
                    height: stretch::style::Dimension::Points(20.0),
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 50.0);
        assert_eq!(layout.children[0].height, 50.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 50.0);
        assert_eq!(layout.children[1].height, 20.0);
        assert_eq!(layout.children[1].x, 50.0);
        assert_eq!(layout.children[1].y, 30.0);
    }

    #[test]
    fn margin_with_sibling_row() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![
                stretch::style::Node {
                    flex_grow: 1.0,
                    margin: stretch::style::Edges {
                        end: stretch::style::Dimension::Points(10.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node { flex_grow: 1.0, ..Default::default() },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 45.0);
        assert_eq!(layout.children[0].height, 100.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 45.0);
        assert_eq!(layout.children[1].height, 100.0);
        assert_eq!(layout.children[1].x, 55.0);
        assert_eq!(layout.children[1].y, 0.0);
    }

    #[test]
    fn wrap_reverse_row_align_content_space_around() {
        let layout = stretch::compute(&stretch::style::Node {
            flexWrap: stretch::style::FlexWrap::WrapReverse,
            align_content: stretch::style::AlignContent::SpaceAround,
            width: stretch::style::Dimension::Points(100.0),
            children: vec![
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(30.0),
                    height: stretch::style::Dimension::Points(10.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(30.0),
                    height: stretch::style::Dimension::Points(20.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(30.0),
                    height: stretch::style::Dimension::Points(30.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(30.0),
                    height: stretch::style::Dimension::Points(40.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(30.0),
                    height: stretch::style::Dimension::Points(50.0),
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 80.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 30.0);
        assert_eq!(layout.children[0].height, 10.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 70.0);

        assert_eq!(layout.children[1].width, 30.0);
        assert_eq!(layout.children[1].height, 20.0);
        assert_eq!(layout.children[1].x, 30.0);
        assert_eq!(layout.children[1].y, 60.0);

        assert_eq!(layout.children[2].width, 30.0);
        assert_eq!(layout.children[2].height, 30.0);
        assert_eq!(layout.children[2].x, 60.0);
        assert_eq!(layout.children[2].y, 50.0);

        assert_eq!(layout.children[3].width, 30.0);
        assert_eq!(layout.children[3].height, 40.0);
        assert_eq!(layout.children[3].x, 0.0);
        assert_eq!(layout.children[3].y, 10.0);

        assert_eq!(layout.children[4].width, 30.0);
        assert_eq!(layout.children[4].height, 50.0);
        assert_eq!(layout.children[4].x, 30.0);
        assert_eq!(layout.children[4].y, 0.0);
    }

    #[test]
    fn wrap_reverse_column() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            flexWrap: stretch::style::FlexWrap::WrapReverse,
            height: stretch::style::Dimension::Points(100.0),
            children: vec![
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(30.0),
                    height: stretch::style::Dimension::Points(30.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(30.0),
                    height: stretch::style::Dimension::Points(30.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(30.0),
                    height: stretch::style::Dimension::Points(30.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(30.0),
                    height: stretch::style::Dimension::Points(30.0),
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 30.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 30.0);
        assert_eq!(layout.children[0].height, 30.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 30.0);
        assert_eq!(layout.children[1].height, 30.0);
        assert_eq!(layout.children[1].x, 0.0);
        assert_eq!(layout.children[1].y, 30.0);

        assert_eq!(layout.children[2].width, 30.0);
        assert_eq!(layout.children[2].height, 30.0);
        assert_eq!(layout.children[2].x, 0.0);
        assert_eq!(layout.children[2].y, 60.0);

        assert_eq!(layout.children[3].width, 30.0);
        assert_eq!(layout.children[3].height, 30.0);
        assert_eq!(layout.children[3].x, -30.0);
        assert_eq!(layout.children[3].y, 0.0);
    }

    #[test]
    fn align_items_center_with_child_margin() {
        let layout = stretch::compute(&stretch::style::Node {
            align_items: stretch::style::AlignItems::Center,
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![stretch::style::Node {
                width: stretch::style::Dimension::Points(10.0),
                height: stretch::style::Dimension::Points(10.0),
                margin: stretch::style::Edges { top: stretch::style::Dimension::Points(10.0), ..Default::default() },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 10.0);
        assert_eq!(layout.children[0].height, 10.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 50.0);
    }

    #[test]
    fn flex_direction_column_reverse() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::ColumnReverse,
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![
                stretch::style::Node { height: stretch::style::Dimension::Points(10.0), ..Default::default() },
                stretch::style::Node { height: stretch::style::Dimension::Points(10.0), ..Default::default() },
                stretch::style::Node { height: stretch::style::Dimension::Points(10.0), ..Default::default() },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 100.0);
        assert_eq!(layout.children[0].height, 10.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 90.0);

        assert_eq!(layout.children[1].width, 100.0);
        assert_eq!(layout.children[1].height, 10.0);
        assert_eq!(layout.children[1].x, 0.0);
        assert_eq!(layout.children[1].y, 80.0);

        assert_eq!(layout.children[2].width, 100.0);
        assert_eq!(layout.children[2].height, 10.0);
        assert_eq!(layout.children[2].x, 0.0);
        assert_eq!(layout.children[2].y, 70.0);
    }

    #[test]
    fn flex_direction_column() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![
                stretch::style::Node { height: stretch::style::Dimension::Points(10.0), ..Default::default() },
                stretch::style::Node { height: stretch::style::Dimension::Points(10.0), ..Default::default() },
                stretch::style::Node { height: stretch::style::Dimension::Points(10.0), ..Default::default() },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 100.0);
        assert_eq!(layout.children[0].height, 10.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 100.0);
        assert_eq!(layout.children[1].height, 10.0);
        assert_eq!(layout.children[1].x, 0.0);
        assert_eq!(layout.children[1].y, 10.0);

        assert_eq!(layout.children[2].width, 100.0);
        assert_eq!(layout.children[2].height, 10.0);
        assert_eq!(layout.children[2].x, 0.0);
        assert_eq!(layout.children[2].y, 20.0);
    }

    #[test]
    fn align_items_center_child_without_margin_bigger_than_parent() {
        let layout = stretch::compute(&stretch::style::Node {
            align_items: stretch::style::AlignItems::Center,
            justify_content: stretch::style::JustifyContent::Center,
            width: stretch::style::Dimension::Points(52.0),
            height: stretch::style::Dimension::Points(52.0),
            children: vec![stretch::style::Node {
                align_items: stretch::style::AlignItems::Center,
                children: vec![stretch::style::Node {
                    width: stretch::style::Dimension::Points(72.0),
                    height: stretch::style::Dimension::Points(72.0),
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 52.0);
        assert_eq!(layout.height, 52.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 72.0);
        assert_eq!(layout.children[0].height, 72.0);
        assert_eq!(layout.children[0].x, -10.0);
        assert_eq!(layout.children[0].y, -10.0);

        assert_eq!(layout.children[0].children[0].width, 72.0);
        assert_eq!(layout.children[0].children[0].height, 72.0);
        assert_eq!(layout.children[0].children[0].x, 0.0);
        assert_eq!(layout.children[0].children[0].y, 0.0);
    }

    #[test]
    fn align_self_flex_end_override_flex_start() {
        let layout = stretch::compute(&stretch::style::Node {
            align_items: stretch::style::AlignItems::FlexStart,
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![stretch::style::Node {
                align_self: stretch::style::AlignSelf::FlexEnd,
                width: stretch::style::Dimension::Points(10.0),
                height: stretch::style::Dimension::Points(10.0),
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 10.0);
        assert_eq!(layout.children[0].height, 10.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 90.0);
    }

    #[test]
    fn rounding_total_fractial() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            width: stretch::style::Dimension::Points(87.4),
            height: stretch::style::Dimension::Points(113.4),
            children: vec![
                stretch::style::Node {
                    flex_grow: 0.7,
                    flex_basis: stretch::style::Dimension::Points(50.3),
                    height: stretch::style::Dimension::Points(20.3),
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 1.6,
                    height: stretch::style::Dimension::Points(10.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 1.1,
                    height: stretch::style::Dimension::Points(10.7),
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 87.0);
        assert_eq!(layout.height, 113.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 87.0);
        assert_eq!(layout.children[0].height, 59.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 87.0);
        assert_eq!(layout.children[1].height, 30.0);
        assert_eq!(layout.children[1].x, 0.0);
        assert_eq!(layout.children[1].y, 59.0);

        assert_eq!(layout.children[2].width, 87.0);
        assert_eq!(layout.children[2].height, 24.0);
        assert_eq!(layout.children[2].x, 0.0);
        assert_eq!(layout.children[2].y, 89.0);
    }

    #[test]
    fn align_baseline_child_multiline() {
        let layout = stretch::compute(&stretch::style::Node {
            align_items: stretch::style::AlignItems::Baseline,
            width: stretch::style::Dimension::Points(100.0),
            children: vec![
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(50.0),
                    height: stretch::style::Dimension::Points(60.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    flexWrap: stretch::style::FlexWrap::Wrap,
                    width: stretch::style::Dimension::Points(50.0),
                    children: vec![
                        stretch::style::Node {
                            width: stretch::style::Dimension::Points(25.0),
                            height: stretch::style::Dimension::Points(20.0),
                            ..Default::default()
                        },
                        stretch::style::Node {
                            width: stretch::style::Dimension::Points(25.0),
                            height: stretch::style::Dimension::Points(10.0),
                            ..Default::default()
                        },
                        stretch::style::Node {
                            width: stretch::style::Dimension::Points(25.0),
                            height: stretch::style::Dimension::Points(20.0),
                            ..Default::default()
                        },
                        stretch::style::Node {
                            width: stretch::style::Dimension::Points(25.0),
                            height: stretch::style::Dimension::Points(10.0),
                            ..Default::default()
                        },
                    ],
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 80.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 50.0);
        assert_eq!(layout.children[0].height, 60.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 50.0);
        assert_eq!(layout.children[1].height, 40.0);
        assert_eq!(layout.children[1].x, 50.0);
        assert_eq!(layout.children[1].y, 40.0);

        assert_eq!(layout.children[1].children[0].width, 25.0);
        assert_eq!(layout.children[1].children[0].height, 20.0);
        assert_eq!(layout.children[1].children[0].x, 0.0);
        assert_eq!(layout.children[1].children[0].y, 0.0);

        assert_eq!(layout.children[1].children[1].width, 25.0);
        assert_eq!(layout.children[1].children[1].height, 10.0);
        assert_eq!(layout.children[1].children[1].x, 25.0);
        assert_eq!(layout.children[1].children[1].y, 0.0);

        assert_eq!(layout.children[1].children[2].width, 25.0);
        assert_eq!(layout.children[1].children[2].height, 20.0);
        assert_eq!(layout.children[1].children[2].x, 0.0);
        assert_eq!(layout.children[1].children[2].y, 20.0);

        assert_eq!(layout.children[1].children[3].width, 25.0);
        assert_eq!(layout.children[1].children[3].height, 10.0);
        assert_eq!(layout.children[1].children[3].x, 25.0);
        assert_eq!(layout.children[1].children[3].y, 20.0);
    }

    #[test]
    fn absolute_layout_align_items_center_on_child_only() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(110.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![stretch::style::Node {
                position: stretch::style::Position::Absolute,
                align_self: stretch::style::AlignSelf::Center,
                width: stretch::style::Dimension::Points(60.0),
                height: stretch::style::Dimension::Points(40.0),
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 110.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 60.0);
        assert_eq!(layout.children[0].height, 40.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 30.0);
    }

    #[test]
    fn wrap_reverse_row_single_line_different_size() {
        let layout = stretch::compute(&stretch::style::Node {
            flexWrap: stretch::style::FlexWrap::WrapReverse,
            align_content: stretch::style::AlignContent::FlexStart,
            width: stretch::style::Dimension::Points(300.0),
            children: vec![
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(30.0),
                    height: stretch::style::Dimension::Points(10.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(30.0),
                    height: stretch::style::Dimension::Points(20.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(30.0),
                    height: stretch::style::Dimension::Points(30.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(30.0),
                    height: stretch::style::Dimension::Points(40.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(30.0),
                    height: stretch::style::Dimension::Points(50.0),
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 300.0);
        assert_eq!(layout.height, 50.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 30.0);
        assert_eq!(layout.children[0].height, 10.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 40.0);

        assert_eq!(layout.children[1].width, 30.0);
        assert_eq!(layout.children[1].height, 20.0);
        assert_eq!(layout.children[1].x, 30.0);
        assert_eq!(layout.children[1].y, 30.0);

        assert_eq!(layout.children[2].width, 30.0);
        assert_eq!(layout.children[2].height, 30.0);
        assert_eq!(layout.children[2].x, 60.0);
        assert_eq!(layout.children[2].y, 20.0);

        assert_eq!(layout.children[3].width, 30.0);
        assert_eq!(layout.children[3].height, 40.0);
        assert_eq!(layout.children[3].x, 90.0);
        assert_eq!(layout.children[3].y, 10.0);

        assert_eq!(layout.children[4].width, 30.0);
        assert_eq!(layout.children[4].height, 50.0);
        assert_eq!(layout.children[4].x, 120.0);
        assert_eq!(layout.children[4].y, 0.0);
    }

    #[test]
    fn rounding_fractial_input_4() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(113.4),
            children: vec![
                stretch::style::Node {
                    flex_grow: 1.0,
                    flex_basis: stretch::style::Dimension::Points(50.0),
                    height: stretch::style::Dimension::Points(20.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 1.0,
                    height: stretch::style::Dimension::Points(10.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 1.0,
                    height: stretch::style::Dimension::Points(10.0),
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 113.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 100.0);
        assert_eq!(layout.children[0].height, 64.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 100.0);
        assert_eq!(layout.children[1].height, 25.0);
        assert_eq!(layout.children[1].x, 0.0);
        assert_eq!(layout.children[1].y, 64.0);

        assert_eq!(layout.children[2].width, 100.0);
        assert_eq!(layout.children[2].height, 24.0);
        assert_eq!(layout.children[2].x, 0.0);
        assert_eq!(layout.children[2].y, 89.0);
    }

    #[test]
    fn padding_flex_child() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            padding: stretch::style::Edges {
                start: stretch::style::Dimension::Points(10.0),
                end: stretch::style::Dimension::Points(10.0),
                top: stretch::style::Dimension::Points(10.0),
                bottom: stretch::style::Dimension::Points(10.0),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                flex_grow: 1.0,
                width: stretch::style::Dimension::Points(10.0),
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 80.0);
        assert_eq!(layout.children[0].height, 80.0);
        assert_eq!(layout.children[0].x, 10.0);
        assert_eq!(layout.children[0].y, 10.0);
    }

    #[test]
    fn flex_direction_row_reverse() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::RowReverse,
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![
                stretch::style::Node { width: stretch::style::Dimension::Points(10.0), ..Default::default() },
                stretch::style::Node { width: stretch::style::Dimension::Points(10.0), ..Default::default() },
                stretch::style::Node { width: stretch::style::Dimension::Points(10.0), ..Default::default() },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 10.0);
        assert_eq!(layout.children[0].height, 100.0);
        assert_eq!(layout.children[0].x, 90.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 10.0);
        assert_eq!(layout.children[1].height, 100.0);
        assert_eq!(layout.children[1].x, 80.0);
        assert_eq!(layout.children[1].y, 0.0);

        assert_eq!(layout.children[2].width, 10.0);
        assert_eq!(layout.children[2].height, 100.0);
        assert_eq!(layout.children[2].x, 70.0);
        assert_eq!(layout.children[2].y, 0.0);
    }

    #[test]
    fn wrap_reverse_row_align_content_stretch() {
        let layout = stretch::compute(&stretch::style::Node {
            flexWrap: stretch::style::FlexWrap::WrapReverse,
            width: stretch::style::Dimension::Points(100.0),
            children: vec![
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(30.0),
                    height: stretch::style::Dimension::Points(10.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(30.0),
                    height: stretch::style::Dimension::Points(20.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(30.0),
                    height: stretch::style::Dimension::Points(30.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(30.0),
                    height: stretch::style::Dimension::Points(40.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(30.0),
                    height: stretch::style::Dimension::Points(50.0),
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 80.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 30.0);
        assert_eq!(layout.children[0].height, 10.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 70.0);

        assert_eq!(layout.children[1].width, 30.0);
        assert_eq!(layout.children[1].height, 20.0);
        assert_eq!(layout.children[1].x, 30.0);
        assert_eq!(layout.children[1].y, 60.0);

        assert_eq!(layout.children[2].width, 30.0);
        assert_eq!(layout.children[2].height, 30.0);
        assert_eq!(layout.children[2].x, 60.0);
        assert_eq!(layout.children[2].y, 50.0);

        assert_eq!(layout.children[3].width, 30.0);
        assert_eq!(layout.children[3].height, 40.0);
        assert_eq!(layout.children[3].x, 0.0);
        assert_eq!(layout.children[3].y, 10.0);

        assert_eq!(layout.children[4].width, 30.0);
        assert_eq!(layout.children[4].height, 50.0);
        assert_eq!(layout.children[4].x, 30.0);
        assert_eq!(layout.children[4].y, 0.0);
    }

    #[test]
    fn margin_auto_top() {
        let layout = stretch::compute(&stretch::style::Node {
            align_items: stretch::style::AlignItems::Center,
            width: stretch::style::Dimension::Points(200.0),
            height: stretch::style::Dimension::Points(200.0),
            children: vec![
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(50.0),
                    height: stretch::style::Dimension::Points(50.0),
                    margin: stretch::style::Edges { top: stretch::style::Dimension::Auto, ..Default::default() },
                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(50.0),
                    height: stretch::style::Dimension::Points(50.0),
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 200.0);
        assert_eq!(layout.height, 200.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 50.0);
        assert_eq!(layout.children[0].height, 50.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 150.0);

        assert_eq!(layout.children[1].width, 50.0);
        assert_eq!(layout.children[1].height, 50.0);
        assert_eq!(layout.children[1].x, 50.0);
        assert_eq!(layout.children[1].y, 75.0);
    }

    #[test]
    fn flex_grow_within_constrained_max_row() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            width: stretch::style::Dimension::Points(200.0),
            children: vec![stretch::style::Node {
                max_width: stretch::style::Dimension::Points(100.0),
                height: stretch::style::Dimension::Points(100.0),
                children: vec![
                    stretch::style::Node {
                        flex_shrink: 1.0,
                        flex_basis: stretch::style::Dimension::Points(100.0),
                        ..Default::default()
                    },
                    stretch::style::Node { width: stretch::style::Dimension::Points(50.0), ..Default::default() },
                ],
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 200.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 100.0);
        assert_eq!(layout.children[0].height, 100.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[0].children[0].width, 67.0);
        assert_eq!(layout.children[0].children[0].height, 100.0);
        assert_eq!(layout.children[0].children[0].x, 0.0);
        assert_eq!(layout.children[0].children[0].y, 0.0);

        assert_eq!(layout.children[0].children[1].width, 33.0);
        assert_eq!(layout.children[0].children[1].height, 100.0);
        assert_eq!(layout.children[0].children[1].x, 67.0);
        assert_eq!(layout.children[0].children[1].y, 0.0);
    }

    #[test]
    fn absolute_layout_align_items_and_justify_content_center() {
        let layout = stretch::compute(&stretch::style::Node {
            align_items: stretch::style::AlignItems::Center,
            justify_content: stretch::style::JustifyContent::Center,
            width: stretch::style::Dimension::Points(110.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![stretch::style::Node {
                position: stretch::style::Position::Absolute,
                width: stretch::style::Dimension::Points(60.0),
                height: stretch::style::Dimension::Points(40.0),
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 110.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 60.0);
        assert_eq!(layout.children[0].height, 40.0);
        assert_eq!(layout.children[0].x, 25.0);
        assert_eq!(layout.children[0].y, 30.0);
    }

    #[test]
    fn align_self_flex_end() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![stretch::style::Node {
                align_self: stretch::style::AlignSelf::FlexEnd,
                width: stretch::style::Dimension::Points(10.0),
                height: stretch::style::Dimension::Points(10.0),
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 10.0);
        assert_eq!(layout.children[0].height, 10.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 90.0);
    }

    #[test]
    fn margin_auto_left_child_bigger_than_parent() {
        let layout = stretch::compute(&stretch::style::Node {
            justify_content: stretch::style::JustifyContent::Center,
            width: stretch::style::Dimension::Points(52.0),
            height: stretch::style::Dimension::Points(52.0),
            children: vec![stretch::style::Node {
                width: stretch::style::Dimension::Points(72.0),
                height: stretch::style::Dimension::Points(72.0),
                margin: stretch::style::Edges { start: stretch::style::Dimension::Auto, ..Default::default() },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 52.0);
        assert_eq!(layout.height, 52.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 52.0);
        assert_eq!(layout.children[0].height, 72.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);
    }

    #[test]
    fn wrapped_row_within_align_items_flex_end() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            align_items: stretch::style::AlignItems::FlexEnd,
            width: stretch::style::Dimension::Points(200.0),
            height: stretch::style::Dimension::Points(200.0),
            children: vec![stretch::style::Node {
                flexWrap: stretch::style::FlexWrap::Wrap,
                children: vec![
                    stretch::style::Node {
                        width: stretch::style::Dimension::Points(150.0),
                        height: stretch::style::Dimension::Points(80.0),
                        ..Default::default()
                    },
                    stretch::style::Node {
                        width: stretch::style::Dimension::Points(80.0),
                        height: stretch::style::Dimension::Points(80.0),
                        ..Default::default()
                    },
                ],
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 200.0);
        assert_eq!(layout.height, 200.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 200.0);
        assert_eq!(layout.children[0].height, 160.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[0].children[0].width, 150.0);
        assert_eq!(layout.children[0].children[0].height, 80.0);
        assert_eq!(layout.children[0].children[0].x, 0.0);
        assert_eq!(layout.children[0].children[0].y, 0.0);

        assert_eq!(layout.children[0].children[1].width, 80.0);
        assert_eq!(layout.children[0].children[1].height, 80.0);
        assert_eq!(layout.children[0].children[1].x, 0.0);
        assert_eq!(layout.children[0].children[1].y, 80.0);
    }

    #[test]
    fn min_height_overrides_height() {
        let layout = stretch::compute(&stretch::style::Node {
            children: vec![stretch::style::Node {
                height: stretch::style::Dimension::Points(50.0),
                min_height: stretch::style::Dimension::Points(100.0),
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 0.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 0.0);
        assert_eq!(layout.children[0].height, 100.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);
    }

    #[test]
    fn align_strech_should_size_based_on_parent() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![stretch::style::Node {
                justify_content: stretch::style::JustifyContent::Center,
                flex_grow: 0.0,
                flex_shrink: 1.0,
                children: vec![stretch::style::Node {
                    flex_grow: 1.0,
                    flex_shrink: 1.0,
                    children: vec![stretch::style::Node {
                        width: stretch::style::Dimension::Points(20.0),
                        height: stretch::style::Dimension::Points(20.0),
                        ..Default::default()
                    }],
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 20.0);
        assert_eq!(layout.children[0].height, 100.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[0].children[0].width, 20.0);
        assert_eq!(layout.children[0].children[0].height, 100.0);
        assert_eq!(layout.children[0].children[0].x, 0.0);
        assert_eq!(layout.children[0].children[0].y, 0.0);

        assert_eq!(layout.children[0].children[0].children[0].width, 20.0);
        assert_eq!(layout.children[0].children[0].children[0].height, 20.0);
        assert_eq!(layout.children[0].children[0].children[0].x, 0.0);
        assert_eq!(layout.children[0].children[0].children[0].y, 0.0);
    }

    #[test]
    fn wrap_row_align_items_flex_end() {
        let layout = stretch::compute(&stretch::style::Node {
            flexWrap: stretch::style::FlexWrap::Wrap,
            align_items: stretch::style::AlignItems::FlexEnd,
            width: stretch::style::Dimension::Points(100.0),
            children: vec![
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(30.0),
                    height: stretch::style::Dimension::Points(10.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(30.0),
                    height: stretch::style::Dimension::Points(20.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(30.0),
                    height: stretch::style::Dimension::Points(30.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(30.0),
                    height: stretch::style::Dimension::Points(30.0),
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 60.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 30.0);
        assert_eq!(layout.children[0].height, 10.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 20.0);

        assert_eq!(layout.children[1].width, 30.0);
        assert_eq!(layout.children[1].height, 20.0);
        assert_eq!(layout.children[1].x, 30.0);
        assert_eq!(layout.children[1].y, 10.0);

        assert_eq!(layout.children[2].width, 30.0);
        assert_eq!(layout.children[2].height, 30.0);
        assert_eq!(layout.children[2].x, 60.0);
        assert_eq!(layout.children[2].y, 0.0);

        assert_eq!(layout.children[3].width, 30.0);
        assert_eq!(layout.children[3].height, 30.0);
        assert_eq!(layout.children[3].x, 0.0);
        assert_eq!(layout.children[3].y, 30.0);
    }

    #[test]
    fn size_defined_by_child_with_border() {
        let layout = stretch::compute(&stretch::style::Node {
            border: stretch::style::Edges {
                start: stretch::style::Dimension::Points(10.0),
                end: stretch::style::Dimension::Points(10.0),
                top: stretch::style::Dimension::Points(10.0),
                bottom: stretch::style::Dimension::Points(10.0),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                width: stretch::style::Dimension::Points(10.0),
                height: stretch::style::Dimension::Points(10.0),
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 30.0);
        assert_eq!(layout.height, 30.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 10.0);
        assert_eq!(layout.children[0].height, 10.0);
        assert_eq!(layout.children[0].x, 10.0);
        assert_eq!(layout.children[0].y, 10.0);
    }

    #[test]
    fn margin_auto_bottom_and_top_justify_center() {
        let layout = stretch::compute(&stretch::style::Node {
            justify_content: stretch::style::JustifyContent::Center,
            width: stretch::style::Dimension::Points(200.0),
            height: stretch::style::Dimension::Points(200.0),
            children: vec![
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(50.0),
                    height: stretch::style::Dimension::Points(50.0),
                    margin: stretch::style::Edges {
                        top: stretch::style::Dimension::Auto,
                        bottom: stretch::style::Dimension::Auto,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(50.0),
                    height: stretch::style::Dimension::Points(50.0),
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 200.0);
        assert_eq!(layout.height, 200.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 50.0);
        assert_eq!(layout.children[0].height, 50.0);
        assert_eq!(layout.children[0].x, 50.0);
        assert_eq!(layout.children[0].y, 75.0);

        assert_eq!(layout.children[1].width, 50.0);
        assert_eq!(layout.children[1].height, 50.0);
        assert_eq!(layout.children[1].x, 100.0);
        assert_eq!(layout.children[1].y, 0.0);
    }

    #[test]
    fn flex_basis_overrides_main_size() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![
                stretch::style::Node {
                    flex_grow: 1.0,
                    flex_basis: stretch::style::Dimension::Points(50.0),
                    width: stretch::style::Dimension::Points(20.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 1.0,
                    width: stretch::style::Dimension::Points(10.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 1.0,
                    width: stretch::style::Dimension::Points(10.0),
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 60.0);
        assert_eq!(layout.children[0].height, 100.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 20.0);
        assert_eq!(layout.children[1].height, 100.0);
        assert_eq!(layout.children[1].x, 60.0);
        assert_eq!(layout.children[1].y, 0.0);

        assert_eq!(layout.children[2].width, 20.0);
        assert_eq!(layout.children[2].height, 100.0);
        assert_eq!(layout.children[2].x, 80.0);
        assert_eq!(layout.children[2].y, 0.0);
    }

    #[test]
    fn wrapped_row_within_align_items_flex_start() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            align_items: stretch::style::AlignItems::FlexStart,
            width: stretch::style::Dimension::Points(200.0),
            height: stretch::style::Dimension::Points(200.0),
            children: vec![stretch::style::Node {
                flexWrap: stretch::style::FlexWrap::Wrap,
                children: vec![
                    stretch::style::Node {
                        width: stretch::style::Dimension::Points(150.0),
                        height: stretch::style::Dimension::Points(80.0),
                        ..Default::default()
                    },
                    stretch::style::Node {
                        width: stretch::style::Dimension::Points(80.0),
                        height: stretch::style::Dimension::Points(80.0),
                        ..Default::default()
                    },
                ],
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 200.0);
        assert_eq!(layout.height, 200.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 200.0);
        assert_eq!(layout.children[0].height, 160.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[0].children[0].width, 150.0);
        assert_eq!(layout.children[0].children[0].height, 80.0);
        assert_eq!(layout.children[0].children[0].x, 0.0);
        assert_eq!(layout.children[0].children[0].y, 0.0);

        assert_eq!(layout.children[0].children[1].width, 80.0);
        assert_eq!(layout.children[0].children[1].height, 80.0);
        assert_eq!(layout.children[0].children[1].x, 0.0);
        assert_eq!(layout.children[0].children[1].y, 80.0);
    }

    #[test]
    fn align_center_should_size_based_on_content() {
        let layout = stretch::compute(&stretch::style::Node {
            align_items: stretch::style::AlignItems::Center,
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            margin: stretch::style::Edges { top: stretch::style::Dimension::Points(20.0), ..Default::default() },
            children: vec![stretch::style::Node {
                justify_content: stretch::style::JustifyContent::Center,
                flex_grow: 0.0,
                flex_shrink: 1.0,
                children: vec![stretch::style::Node {
                    flex_grow: 1.0,
                    flex_shrink: 1.0,
                    children: vec![stretch::style::Node {
                        width: stretch::style::Dimension::Points(20.0),
                        height: stretch::style::Dimension::Points(20.0),
                        ..Default::default()
                    }],
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 20.0);

        assert_eq!(layout.children[0].width, 20.0);
        assert_eq!(layout.children[0].height, 20.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 40.0);

        assert_eq!(layout.children[0].children[0].width, 20.0);
        assert_eq!(layout.children[0].children[0].height, 20.0);
        assert_eq!(layout.children[0].children[0].x, 0.0);
        assert_eq!(layout.children[0].children[0].y, 0.0);

        assert_eq!(layout.children[0].children[0].children[0].width, 20.0);
        assert_eq!(layout.children[0].children[0].children[0].height, 20.0);
        assert_eq!(layout.children[0].children[0].children[0].x, 0.0);
        assert_eq!(layout.children[0].children[0].children[0].y, 0.0);
    }

    #[test]
    fn margin_auto_right() {
        let layout = stretch::compute(&stretch::style::Node {
            align_items: stretch::style::AlignItems::Center,
            width: stretch::style::Dimension::Points(200.0),
            height: stretch::style::Dimension::Points(200.0),
            children: vec![
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(50.0),
                    height: stretch::style::Dimension::Points(50.0),
                    margin: stretch::style::Edges { end: stretch::style::Dimension::Auto, ..Default::default() },
                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(50.0),
                    height: stretch::style::Dimension::Points(50.0),
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 200.0);
        assert_eq!(layout.height, 200.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 50.0);
        assert_eq!(layout.children[0].height, 50.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 75.0);

        assert_eq!(layout.children[1].width, 50.0);
        assert_eq!(layout.children[1].height, 50.0);
        assert_eq!(layout.children[1].x, 150.0);
        assert_eq!(layout.children[1].y, 75.0);
    }

    #[test]
    fn wrap_reverse_row_align_content_center() {
        let layout = stretch::compute(&stretch::style::Node {
            flexWrap: stretch::style::FlexWrap::WrapReverse,
            align_content: stretch::style::AlignContent::Center,
            width: stretch::style::Dimension::Points(100.0),
            children: vec![
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(30.0),
                    height: stretch::style::Dimension::Points(10.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(30.0),
                    height: stretch::style::Dimension::Points(20.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(30.0),
                    height: stretch::style::Dimension::Points(30.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(30.0),
                    height: stretch::style::Dimension::Points(40.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(30.0),
                    height: stretch::style::Dimension::Points(50.0),
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 80.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 30.0);
        assert_eq!(layout.children[0].height, 10.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 70.0);

        assert_eq!(layout.children[1].width, 30.0);
        assert_eq!(layout.children[1].height, 20.0);
        assert_eq!(layout.children[1].x, 30.0);
        assert_eq!(layout.children[1].y, 60.0);

        assert_eq!(layout.children[2].width, 30.0);
        assert_eq!(layout.children[2].height, 30.0);
        assert_eq!(layout.children[2].x, 60.0);
        assert_eq!(layout.children[2].y, 50.0);

        assert_eq!(layout.children[3].width, 30.0);
        assert_eq!(layout.children[3].height, 40.0);
        assert_eq!(layout.children[3].x, 0.0);
        assert_eq!(layout.children[3].y, 10.0);

        assert_eq!(layout.children[4].width, 30.0);
        assert_eq!(layout.children[4].height, 50.0);
        assert_eq!(layout.children[4].x, 30.0);
        assert_eq!(layout.children[4].y, 0.0);
    }

    #[test]
    fn padding_no_child() {
        let layout = stretch::compute(&stretch::style::Node {
            padding: stretch::style::Edges {
                start: stretch::style::Dimension::Points(10.0),
                end: stretch::style::Dimension::Points(10.0),
                top: stretch::style::Dimension::Points(10.0),
                bottom: stretch::style::Dimension::Points(10.0),
                ..Default::default()
            },
            ..Default::default()
        });

        assert_eq!(layout.width, 20.0);
        assert_eq!(layout.height, 20.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);
    }

    #[test]
    fn margin_with_sibling_column() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![
                stretch::style::Node {
                    flex_grow: 1.0,
                    margin: stretch::style::Edges {
                        bottom: stretch::style::Dimension::Points(10.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node { flex_grow: 1.0, ..Default::default() },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 100.0);
        assert_eq!(layout.children[0].height, 45.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 100.0);
        assert_eq!(layout.children[1].height, 45.0);
        assert_eq!(layout.children[1].x, 0.0);
        assert_eq!(layout.children[1].y, 55.0);
    }

    #[test]
    fn flex_direction_row_no_width() {
        let layout = stretch::compute(&stretch::style::Node {
            height: stretch::style::Dimension::Points(100.0),
            children: vec![
                stretch::style::Node { width: stretch::style::Dimension::Points(10.0), ..Default::default() },
                stretch::style::Node { width: stretch::style::Dimension::Points(10.0), ..Default::default() },
                stretch::style::Node { width: stretch::style::Dimension::Points(10.0), ..Default::default() },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 30.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 10.0);
        assert_eq!(layout.children[0].height, 100.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 10.0);
        assert_eq!(layout.children[1].height, 100.0);
        assert_eq!(layout.children[1].x, 10.0);
        assert_eq!(layout.children[1].y, 0.0);

        assert_eq!(layout.children[2].width, 10.0);
        assert_eq!(layout.children[2].height, 100.0);
        assert_eq!(layout.children[2].x, 20.0);
        assert_eq!(layout.children[2].y, 0.0);
    }

    #[test]
    fn max_width_overrides_width_on_root() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(200.0),
            max_width: stretch::style::Dimension::Points(100.0),
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 0.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);
    }

    #[test]
    fn margin_auto_left_and_right() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(200.0),
            height: stretch::style::Dimension::Points(200.0),
            children: vec![
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(50.0),
                    height: stretch::style::Dimension::Points(50.0),
                    margin: stretch::style::Edges {
                        start: stretch::style::Dimension::Auto,
                        end: stretch::style::Dimension::Auto,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(50.0),
                    height: stretch::style::Dimension::Points(50.0),
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 200.0);
        assert_eq!(layout.height, 200.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 50.0);
        assert_eq!(layout.children[0].height, 50.0);
        assert_eq!(layout.children[0].x, 50.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 50.0);
        assert_eq!(layout.children[1].height, 50.0);
        assert_eq!(layout.children[1].x, 150.0);
        assert_eq!(layout.children[1].y, 0.0);
    }

    #[test]
    fn flex_direction_column_no_height() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            width: stretch::style::Dimension::Points(100.0),
            children: vec![
                stretch::style::Node { height: stretch::style::Dimension::Points(10.0), ..Default::default() },
                stretch::style::Node { height: stretch::style::Dimension::Points(10.0), ..Default::default() },
                stretch::style::Node { height: stretch::style::Dimension::Points(10.0), ..Default::default() },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 30.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 100.0);
        assert_eq!(layout.children[0].height, 10.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 100.0);
        assert_eq!(layout.children[1].height, 10.0);
        assert_eq!(layout.children[1].x, 0.0);
        assert_eq!(layout.children[1].y, 10.0);

        assert_eq!(layout.children[2].width, 100.0);
        assert_eq!(layout.children[2].height, 10.0);
        assert_eq!(layout.children[2].x, 0.0);
        assert_eq!(layout.children[2].y, 20.0);
    }

    #[test]
    fn absolute_layout_in_wrap_reverse_row_container() {
        let layout = stretch::compute(&stretch::style::Node {
            flexWrap: stretch::style::FlexWrap::WrapReverse,
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![stretch::style::Node {
                position: stretch::style::Position::Absolute,
                width: stretch::style::Dimension::Points(20.0),
                height: stretch::style::Dimension::Points(20.0),
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 20.0);
        assert_eq!(layout.children[0].height, 20.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 80.0);
    }

    #[test]
    fn percentage_margin_should_calculate_based_only_on_width() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            width: stretch::style::Dimension::Points(200.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![stretch::style::Node {
                flex_direction: stretch::style::FlexDirection::Column,
                flex_grow: 1.0,
                margin: stretch::style::Edges {
                    start: stretch::style::Dimension::Percent(0.1),
                    end: stretch::style::Dimension::Percent(0.1),
                    top: stretch::style::Dimension::Percent(0.1),
                    bottom: stretch::style::Dimension::Percent(0.1),
                    ..Default::default()
                },
                children: vec![stretch::style::Node {
                    width: stretch::style::Dimension::Points(10.0),
                    height: stretch::style::Dimension::Points(10.0),
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 200.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 160.0);
        assert_eq!(layout.children[0].height, 60.0);
        assert_eq!(layout.children[0].x, 20.0);
        assert_eq!(layout.children[0].y, 20.0);

        assert_eq!(layout.children[0].children[0].width, 10.0);
        assert_eq!(layout.children[0].children[0].height, 10.0);
        assert_eq!(layout.children[0].children[0].x, 0.0);
        assert_eq!(layout.children[0].children[0].y, 0.0);
    }

    #[test]
    fn absolute_layout_start_top_end_bottom() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![stretch::style::Node {
                position: stretch::style::Position::Absolute,
                start: stretch::style::Dimension::Points(10.0),
                end: stretch::style::Dimension::Points(10.0),
                top: stretch::style::Dimension::Points(10.0),
                bottom: stretch::style::Dimension::Points(10.0),
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 80.0);
        assert_eq!(layout.children[0].height, 80.0);
        assert_eq!(layout.children[0].x, 10.0);
        assert_eq!(layout.children[0].y, 10.0);
    }

    #[test]
    fn min_width() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![
                stretch::style::Node {
                    flex_grow: 1.0,
                    min_width: stretch::style::Dimension::Points(60.0),
                    ..Default::default()
                },
                stretch::style::Node { flex_grow: 1.0, ..Default::default() },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 80.0);
        assert_eq!(layout.children[0].height, 100.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 20.0);
        assert_eq!(layout.children[1].height, 100.0);
        assert_eq!(layout.children[1].x, 80.0);
        assert_eq!(layout.children[1].y, 0.0);
    }

    #[test]
    fn min_height_overrides_height_on_root() {
        let layout = stretch::compute(&stretch::style::Node {
            height: stretch::style::Dimension::Points(50.0),
            min_height: stretch::style::Dimension::Points(100.0),
            ..Default::default()
        });

        assert_eq!(layout.width, 0.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);
    }

    #[test]
    fn absolute_layout_width_height_end_bottom() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![stretch::style::Node {
                position: stretch::style::Position::Absolute,
                width: stretch::style::Dimension::Points(10.0),
                height: stretch::style::Dimension::Points(10.0),
                end: stretch::style::Dimension::Points(10.0),
                bottom: stretch::style::Dimension::Points(10.0),
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 10.0);
        assert_eq!(layout.children[0].height, 10.0);
        assert_eq!(layout.children[0].x, 80.0);
        assert_eq!(layout.children[0].y, 80.0);
    }

    #[test]
    fn align_self_center() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![stretch::style::Node {
                align_self: stretch::style::AlignSelf::Center,
                width: stretch::style::Dimension::Points(10.0),
                height: stretch::style::Dimension::Points(10.0),
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 10.0);
        assert_eq!(layout.children[0].height, 10.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 45.0);
    }

    #[test]
    fn wrapped_column_max_height() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            flexWrap: stretch::style::FlexWrap::Wrap,
            align_items: stretch::style::AlignItems::Center,
            align_content: stretch::style::AlignContent::Center,
            justify_content: stretch::style::JustifyContent::Center,
            width: stretch::style::Dimension::Points(700.0),
            height: stretch::style::Dimension::Points(500.0),
            children: vec![
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(100.0),
                    height: stretch::style::Dimension::Points(500.0),
                    max_height: stretch::style::Dimension::Points(200.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(200.0),
                    height: stretch::style::Dimension::Points(200.0),
                    margin: stretch::style::Edges {
                        start: stretch::style::Dimension::Points(20.0),
                        end: stretch::style::Dimension::Points(20.0),
                        top: stretch::style::Dimension::Points(20.0),
                        bottom: stretch::style::Dimension::Points(20.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(100.0),
                    height: stretch::style::Dimension::Points(100.0),
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 700.0);
        assert_eq!(layout.height, 500.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 100.0);
        assert_eq!(layout.children[0].height, 200.0);
        assert_eq!(layout.children[0].x, 250.0);
        assert_eq!(layout.children[0].y, 30.0);

        assert_eq!(layout.children[1].width, 200.0);
        assert_eq!(layout.children[1].height, 200.0);
        assert_eq!(layout.children[1].x, 200.0);
        assert_eq!(layout.children[1].y, 250.0);

        assert_eq!(layout.children[2].width, 100.0);
        assert_eq!(layout.children[2].height, 100.0);
        assert_eq!(layout.children[2].x, 420.0);
        assert_eq!(layout.children[2].y, 200.0);
    }

    #[test]
    fn wrap_row() {
        let layout = stretch::compute(&stretch::style::Node {
            flexWrap: stretch::style::FlexWrap::Wrap,
            width: stretch::style::Dimension::Points(100.0),
            children: vec![
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(30.0),
                    height: stretch::style::Dimension::Points(30.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(30.0),
                    height: stretch::style::Dimension::Points(30.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(30.0),
                    height: stretch::style::Dimension::Points(30.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(30.0),
                    height: stretch::style::Dimension::Points(30.0),
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 60.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 30.0);
        assert_eq!(layout.children[0].height, 30.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 30.0);
        assert_eq!(layout.children[1].height, 30.0);
        assert_eq!(layout.children[1].x, 30.0);
        assert_eq!(layout.children[1].y, 0.0);

        assert_eq!(layout.children[2].width, 30.0);
        assert_eq!(layout.children[2].height, 30.0);
        assert_eq!(layout.children[2].x, 60.0);
        assert_eq!(layout.children[2].y, 0.0);

        assert_eq!(layout.children[3].width, 30.0);
        assert_eq!(layout.children[3].height, 30.0);
        assert_eq!(layout.children[3].x, 0.0);
        assert_eq!(layout.children[3].y, 30.0);
    }

    #[test]
    fn max_height() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![stretch::style::Node {
                width: stretch::style::Dimension::Points(10.0),
                max_height: stretch::style::Dimension::Points(50.0),
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 10.0);
        assert_eq!(layout.children[0].height, 50.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);
    }

    #[test]
    fn flex_grow_child() {
        let layout = stretch::compute(&stretch::style::Node {
            children: vec![stretch::style::Node {
                flex_grow: 1.0,
                flex_basis: stretch::style::Dimension::Points(0.0),
                height: stretch::style::Dimension::Points(100.0),
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 0.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 0.0);
        assert_eq!(layout.children[0].height, 100.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);
    }

    #[test]
    fn align_items_flex_end_child_with_margin_bigger_than_parent() {
        let layout = stretch::compute(&stretch::style::Node {
            align_items: stretch::style::AlignItems::Center,
            justify_content: stretch::style::JustifyContent::Center,
            width: stretch::style::Dimension::Points(52.0),
            height: stretch::style::Dimension::Points(52.0),
            children: vec![stretch::style::Node {
                align_items: stretch::style::AlignItems::FlexEnd,
                children: vec![stretch::style::Node {
                    width: stretch::style::Dimension::Points(52.0),
                    height: stretch::style::Dimension::Points(52.0),
                    margin: stretch::style::Edges {
                        start: stretch::style::Dimension::Points(10.0),
                        end: stretch::style::Dimension::Points(10.0),
                        ..Default::default()
                    },
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 52.0);
        assert_eq!(layout.height, 52.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 72.0);
        assert_eq!(layout.children[0].height, 52.0);
        assert_eq!(layout.children[0].x, -10.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[0].children[0].width, 52.0);
        assert_eq!(layout.children[0].children[0].height, 52.0);
        assert_eq!(layout.children[0].children[0].x, 10.0);
        assert_eq!(layout.children[0].children[0].y, 0.0);
    }

    #[test]
    fn justify_content_row_max_width_and_margin() {
        let layout = stretch::compute(&stretch::style::Node {
            justify_content: stretch::style::JustifyContent::Center,
            width: stretch::style::Dimension::Points(100.0),
            max_width: stretch::style::Dimension::Points(80.0),
            margin: stretch::style::Edges { start: stretch::style::Dimension::Points(100.0), ..Default::default() },
            children: vec![stretch::style::Node {
                width: stretch::style::Dimension::Points(20.0),
                height: stretch::style::Dimension::Points(20.0),
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 80.0);
        assert_eq!(layout.height, 20.0);
        assert_eq!(layout.x, 100.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 20.0);
        assert_eq!(layout.children[0].height, 20.0);
        assert_eq!(layout.children[0].x, 30.0);
        assert_eq!(layout.children[0].y, 0.0);
    }

    #[test]
    fn margin_left() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![stretch::style::Node {
                width: stretch::style::Dimension::Points(10.0),
                margin: stretch::style::Edges { start: stretch::style::Dimension::Points(10.0), ..Default::default() },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 10.0);
        assert_eq!(layout.children[0].height, 100.0);
        assert_eq!(layout.children[0].x, 10.0);
        assert_eq!(layout.children[0].y, 0.0);
    }

    #[test]
    fn size_defined_by_child() {
        let layout = stretch::compute(&stretch::style::Node {
            children: vec![stretch::style::Node {
                width: stretch::style::Dimension::Points(100.0),
                height: stretch::style::Dimension::Points(100.0),
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 100.0);
        assert_eq!(layout.children[0].height, 100.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);
    }

    #[test]
    fn rounding_total_fractial_nested() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            width: stretch::style::Dimension::Points(87.4),
            height: stretch::style::Dimension::Points(113.4),
            children: vec![
                stretch::style::Node {
                    flex_direction: stretch::style::FlexDirection::Column,
                    flex_grow: 0.7,
                    flex_basis: stretch::style::Dimension::Points(50.3),
                    height: stretch::style::Dimension::Points(20.3),
                    children: vec![
                        stretch::style::Node {
                            flex_grow: 1.0,
                            flex_basis: stretch::style::Dimension::Points(0.3),
                            height: stretch::style::Dimension::Points(9.9),
                            bottom: stretch::style::Dimension::Points(13.3),
                            ..Default::default()
                        },
                        stretch::style::Node {
                            flex_grow: 4.0,
                            flex_basis: stretch::style::Dimension::Points(0.3),
                            height: stretch::style::Dimension::Points(1.1),
                            top: stretch::style::Dimension::Points(13.3),
                            ..Default::default()
                        },
                    ],
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 1.6,
                    height: stretch::style::Dimension::Points(10.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 1.1,
                    height: stretch::style::Dimension::Points(10.7),
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 87.0);
        assert_eq!(layout.height, 113.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 87.0);
        assert_eq!(layout.children[0].height, 59.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[0].children[0].width, 87.0);
        assert_eq!(layout.children[0].children[0].height, 12.0);
        assert_eq!(layout.children[0].children[0].x, 0.0);
        assert_eq!(layout.children[0].children[0].y, -13.0);

        assert_eq!(layout.children[0].children[1].width, 87.0);
        assert_eq!(layout.children[0].children[1].height, 47.0);
        assert_eq!(layout.children[0].children[1].x, 0.0);
        assert_eq!(layout.children[0].children[1].y, 25.0);

        assert_eq!(layout.children[1].width, 87.0);
        assert_eq!(layout.children[1].height, 30.0);
        assert_eq!(layout.children[1].x, 0.0);
        assert_eq!(layout.children[1].y, 59.0);

        assert_eq!(layout.children[2].width, 87.0);
        assert_eq!(layout.children[2].height, 24.0);
        assert_eq!(layout.children[2].x, 0.0);
        assert_eq!(layout.children[2].y, 89.0);
    }

    #[test]
    fn min_max_percent_no_width_height() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            align_items: stretch::style::AlignItems::FlexStart,
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![stretch::style::Node {
                min_width: stretch::style::Dimension::Percent(0.1),
                max_width: stretch::style::Dimension::Percent(0.1),
                min_height: stretch::style::Dimension::Percent(0.1),
                max_height: stretch::style::Dimension::Percent(0.1),
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 10.0);
        assert_eq!(layout.children[0].height, 10.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);
    }

    #[test]
    fn percent_within_flex_grow() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(350.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![
                stretch::style::Node { width: stretch::style::Dimension::Points(100.0), ..Default::default() },
                stretch::style::Node {
                    flex_direction: stretch::style::FlexDirection::Column,
                    flex_grow: 1.0,
                    children: vec![stretch::style::Node {
                        width: stretch::style::Dimension::Percent(1.0),
                        ..Default::default()
                    }],
                    ..Default::default()
                },
                stretch::style::Node { width: stretch::style::Dimension::Points(100.0), ..Default::default() },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 350.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 100.0);
        assert_eq!(layout.children[0].height, 100.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 150.0);
        assert_eq!(layout.children[1].height, 100.0);
        assert_eq!(layout.children[1].x, 100.0);
        assert_eq!(layout.children[1].y, 0.0);

        assert_eq!(layout.children[1].children[0].width, 150.0);
        assert_eq!(layout.children[1].children[0].height, 0.0);
        assert_eq!(layout.children[1].children[0].x, 0.0);
        assert_eq!(layout.children[1].children[0].y, 0.0);

        assert_eq!(layout.children[2].width, 100.0);
        assert_eq!(layout.children[2].height, 100.0);
        assert_eq!(layout.children[2].x, 250.0);
        assert_eq!(layout.children[2].y, 0.0);
    }

    #[test]
    fn margin_and_stretch_row() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![stretch::style::Node {
                flex_grow: 1.0,
                margin: stretch::style::Edges {
                    top: stretch::style::Dimension::Points(10.0),
                    bottom: stretch::style::Dimension::Points(10.0),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 100.0);
        assert_eq!(layout.children[0].height, 80.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 10.0);
    }

    #[test]
    fn margin_top() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![stretch::style::Node {
                height: stretch::style::Dimension::Points(10.0),
                margin: stretch::style::Edges { top: stretch::style::Dimension::Points(10.0), ..Default::default() },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 100.0);
        assert_eq!(layout.children[0].height, 10.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 10.0);
    }

    #[test]
    fn percentage_flex_basis_main_max_height() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(200.0),
            height: stretch::style::Dimension::Points(200.0),
            children: vec![
                stretch::style::Node {
                    flex_grow: 1.0,
                    flex_basis: stretch::style::Dimension::Percent(0.1),
                    max_height: stretch::style::Dimension::Percent(0.6),
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 4.0,
                    flex_basis: stretch::style::Dimension::Percent(0.1),
                    max_height: stretch::style::Dimension::Percent(0.2),
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 200.0);
        assert_eq!(layout.height, 200.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 52.0);
        assert_eq!(layout.children[0].height, 120.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 148.0);
        assert_eq!(layout.children[1].height, 40.0);
        assert_eq!(layout.children[1].x, 52.0);
        assert_eq!(layout.children[1].y, 0.0);
    }

    #[test]
    fn margin_auto_left_and_right_column() {
        let layout = stretch::compute(&stretch::style::Node {
            align_items: stretch::style::AlignItems::Center,
            width: stretch::style::Dimension::Points(200.0),
            height: stretch::style::Dimension::Points(200.0),
            children: vec![
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(50.0),
                    height: stretch::style::Dimension::Points(50.0),
                    margin: stretch::style::Edges {
                        start: stretch::style::Dimension::Auto,
                        end: stretch::style::Dimension::Auto,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(50.0),
                    height: stretch::style::Dimension::Points(50.0),
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 200.0);
        assert_eq!(layout.height, 200.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 50.0);
        assert_eq!(layout.children[0].height, 50.0);
        assert_eq!(layout.children[0].x, 50.0);
        assert_eq!(layout.children[0].y, 75.0);

        assert_eq!(layout.children[1].width, 50.0);
        assert_eq!(layout.children[1].height, 50.0);
        assert_eq!(layout.children[1].x, 150.0);
        assert_eq!(layout.children[1].y, 75.0);
    }

    #[test]
    fn margin_and_flex_row() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![stretch::style::Node {
                flex_grow: 1.0,
                margin: stretch::style::Edges {
                    start: stretch::style::Dimension::Points(10.0),
                    end: stretch::style::Dimension::Points(10.0),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 80.0);
        assert_eq!(layout.children[0].height, 100.0);
        assert_eq!(layout.children[0].x, 10.0);
        assert_eq!(layout.children[0].y, 0.0);
    }

    #[test]
    fn flex_wrap_children_with_min_main_overriding_flex_basis() {
        let layout = stretch::compute(&stretch::style::Node {
            flexWrap: stretch::style::FlexWrap::Wrap,
            width: stretch::style::Dimension::Points(100.0),
            children: vec![
                stretch::style::Node {
                    flex_basis: stretch::style::Dimension::Points(50.0),
                    min_width: stretch::style::Dimension::Points(55.0),
                    height: stretch::style::Dimension::Points(50.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_basis: stretch::style::Dimension::Points(50.0),
                    min_width: stretch::style::Dimension::Points(55.0),
                    height: stretch::style::Dimension::Points(50.0),
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 55.0);
        assert_eq!(layout.children[0].height, 50.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 55.0);
        assert_eq!(layout.children[1].height, 50.0);
        assert_eq!(layout.children[1].x, 0.0);
        assert_eq!(layout.children[1].y, 50.0);
    }

    #[test]
    fn wrap_row_align_items_center() {
        let layout = stretch::compute(&stretch::style::Node {
            flexWrap: stretch::style::FlexWrap::Wrap,
            align_items: stretch::style::AlignItems::Center,
            width: stretch::style::Dimension::Points(100.0),
            children: vec![
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(30.0),
                    height: stretch::style::Dimension::Points(10.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(30.0),
                    height: stretch::style::Dimension::Points(20.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(30.0),
                    height: stretch::style::Dimension::Points(30.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(30.0),
                    height: stretch::style::Dimension::Points(30.0),
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 60.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 30.0);
        assert_eq!(layout.children[0].height, 10.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 10.0);

        assert_eq!(layout.children[1].width, 30.0);
        assert_eq!(layout.children[1].height, 20.0);
        assert_eq!(layout.children[1].x, 30.0);
        assert_eq!(layout.children[1].y, 5.0);

        assert_eq!(layout.children[2].width, 30.0);
        assert_eq!(layout.children[2].height, 30.0);
        assert_eq!(layout.children[2].x, 60.0);
        assert_eq!(layout.children[2].y, 0.0);

        assert_eq!(layout.children[3].width, 30.0);
        assert_eq!(layout.children[3].height, 30.0);
        assert_eq!(layout.children[3].x, 0.0);
        assert_eq!(layout.children[3].y, 30.0);
    }

    #[test]
    fn justify_content_colunn_max_height_and_margin() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            justify_content: stretch::style::JustifyContent::Center,
            height: stretch::style::Dimension::Points(100.0),
            max_height: stretch::style::Dimension::Points(80.0),
            margin: stretch::style::Edges { top: stretch::style::Dimension::Points(100.0), ..Default::default() },
            children: vec![stretch::style::Node {
                width: stretch::style::Dimension::Points(20.0),
                height: stretch::style::Dimension::Points(20.0),
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 20.0);
        assert_eq!(layout.height, 80.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 100.0);

        assert_eq!(layout.children[0].width, 20.0);
        assert_eq!(layout.children[0].height, 20.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 30.0);
    }

    #[test]
    fn flex_basis_flex_grow_column() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![
                stretch::style::Node {
                    flex_grow: 1.0,
                    flex_basis: stretch::style::Dimension::Points(50.0),
                    ..Default::default()
                },
                stretch::style::Node { flex_grow: 1.0, ..Default::default() },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 100.0);
        assert_eq!(layout.children[0].height, 75.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 100.0);
        assert_eq!(layout.children[1].height, 25.0);
        assert_eq!(layout.children[1].x, 0.0);
        assert_eq!(layout.children[1].y, 75.0);
    }

    #[test]
    fn absolute_layout_align_items_and_justify_content_center_and_right_position() {
        let layout = stretch::compute(&stretch::style::Node {
            align_items: stretch::style::AlignItems::Center,
            justify_content: stretch::style::JustifyContent::Center,
            width: stretch::style::Dimension::Points(110.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![stretch::style::Node {
                position: stretch::style::Position::Absolute,
                width: stretch::style::Dimension::Points(60.0),
                height: stretch::style::Dimension::Points(40.0),
                end: stretch::style::Dimension::Points(5.0),
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 110.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 60.0);
        assert_eq!(layout.children[0].height, 40.0);
        assert_eq!(layout.children[0].x, 45.0);
        assert_eq!(layout.children[0].y, 30.0);
    }

    #[test]
    fn overflow_cross_axis() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![stretch::style::Node {
                width: stretch::style::Dimension::Points(100.0),
                height: stretch::style::Dimension::Points(200.0),
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 100.0);
        assert_eq!(layout.children[0].height, 200.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);
    }

    #[test]
    fn wrap_column() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            flexWrap: stretch::style::FlexWrap::Wrap,
            height: stretch::style::Dimension::Points(100.0),
            children: vec![
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(30.0),
                    height: stretch::style::Dimension::Points(30.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(30.0),
                    height: stretch::style::Dimension::Points(30.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(30.0),
                    height: stretch::style::Dimension::Points(30.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(30.0),
                    height: stretch::style::Dimension::Points(30.0),
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 30.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 30.0);
        assert_eq!(layout.children[0].height, 30.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 30.0);
        assert_eq!(layout.children[1].height, 30.0);
        assert_eq!(layout.children[1].x, 0.0);
        assert_eq!(layout.children[1].y, 30.0);

        assert_eq!(layout.children[2].width, 30.0);
        assert_eq!(layout.children[2].height, 30.0);
        assert_eq!(layout.children[2].x, 0.0);
        assert_eq!(layout.children[2].y, 60.0);

        assert_eq!(layout.children[3].width, 30.0);
        assert_eq!(layout.children[3].height, 30.0);
        assert_eq!(layout.children[3].x, 30.0);
        assert_eq!(layout.children[3].y, 0.0);
    }

    #[test]
    fn justify_content_column_space_evenly() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            justify_content: stretch::style::JustifyContent::SpaceEvenly,
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![
                stretch::style::Node { height: stretch::style::Dimension::Points(10.0), ..Default::default() },
                stretch::style::Node { height: stretch::style::Dimension::Points(10.0), ..Default::default() },
                stretch::style::Node { height: stretch::style::Dimension::Points(10.0), ..Default::default() },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 100.0);
        assert_eq!(layout.children[0].height, 10.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 18.0);

        assert_eq!(layout.children[1].width, 100.0);
        assert_eq!(layout.children[1].height, 10.0);
        assert_eq!(layout.children[1].x, 0.0);
        assert_eq!(layout.children[1].y, 45.0);

        assert_eq!(layout.children[2].width, 100.0);
        assert_eq!(layout.children[2].height, 10.0);
        assert_eq!(layout.children[2].x, 0.0);
        assert_eq!(layout.children[2].y, 73.0);
    }

    #[test]
    fn flex_grow_within_constrained_max_column() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            width: stretch::style::Dimension::Points(100.0),
            max_height: stretch::style::Dimension::Points(100.0),
            children: vec![
                stretch::style::Node {
                    flex_shrink: 1.0,
                    flex_basis: stretch::style::Dimension::Points(100.0),
                    ..Default::default()
                },
                stretch::style::Node { height: stretch::style::Dimension::Points(50.0), ..Default::default() },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 100.0);
        assert_eq!(layout.children[0].height, 67.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 100.0);
        assert_eq!(layout.children[1].height, 33.0);
        assert_eq!(layout.children[1].x, 0.0);
        assert_eq!(layout.children[1].y, 67.0);
    }

    #[test]
    fn justify_content_column_space_around() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            justify_content: stretch::style::JustifyContent::SpaceAround,
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![
                stretch::style::Node { height: stretch::style::Dimension::Points(10.0), ..Default::default() },
                stretch::style::Node { height: stretch::style::Dimension::Points(10.0), ..Default::default() },
                stretch::style::Node { height: stretch::style::Dimension::Points(10.0), ..Default::default() },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 100.0);
        assert_eq!(layout.children[0].height, 10.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 12.0);

        assert_eq!(layout.children[1].width, 100.0);
        assert_eq!(layout.children[1].height, 10.0);
        assert_eq!(layout.children[1].x, 0.0);
        assert_eq!(layout.children[1].y, 45.0);

        assert_eq!(layout.children[2].width, 100.0);
        assert_eq!(layout.children[2].height, 10.0);
        assert_eq!(layout.children[2].x, 0.0);
        assert_eq!(layout.children[2].y, 78.0);
    }

    #[test]
    fn absolute_layout_align_items_and_justify_content_center_and_left_position() {
        let layout = stretch::compute(&stretch::style::Node {
            align_items: stretch::style::AlignItems::Center,
            justify_content: stretch::style::JustifyContent::Center,
            width: stretch::style::Dimension::Points(110.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![stretch::style::Node {
                position: stretch::style::Position::Absolute,
                width: stretch::style::Dimension::Points(60.0),
                height: stretch::style::Dimension::Points(40.0),
                start: stretch::style::Dimension::Points(5.0),
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 110.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 60.0);
        assert_eq!(layout.children[0].height, 40.0);
        assert_eq!(layout.children[0].x, 5.0);
        assert_eq!(layout.children[0].y, 30.0);
    }

    #[test]
    fn percentage_flex_basis_main_min_width() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(200.0),
            height: stretch::style::Dimension::Points(200.0),
            children: vec![
                stretch::style::Node {
                    flex_grow: 1.0,
                    flex_basis: stretch::style::Dimension::Percent(0.2),
                    min_width: stretch::style::Dimension::Percent(0.6),
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 4.0,
                    flex_basis: stretch::style::Dimension::Percent(0.1),
                    min_width: stretch::style::Dimension::Percent(0.2),
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 200.0);
        assert_eq!(layout.height, 200.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 120.0);
        assert_eq!(layout.children[0].height, 200.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 80.0);
        assert_eq!(layout.children[1].height, 200.0);
        assert_eq!(layout.children[1].x, 120.0);
        assert_eq!(layout.children[1].y, 0.0);
    }

    #[test]
    fn margin_auto_bottom() {
        let layout = stretch::compute(&stretch::style::Node {
            align_items: stretch::style::AlignItems::Center,
            width: stretch::style::Dimension::Points(200.0),
            height: stretch::style::Dimension::Points(200.0),
            children: vec![
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(50.0),
                    height: stretch::style::Dimension::Points(50.0),
                    margin: stretch::style::Edges { bottom: stretch::style::Dimension::Auto, ..Default::default() },
                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(50.0),
                    height: stretch::style::Dimension::Points(50.0),
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 200.0);
        assert_eq!(layout.height, 200.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 50.0);
        assert_eq!(layout.children[0].height, 50.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 50.0);
        assert_eq!(layout.children[1].height, 50.0);
        assert_eq!(layout.children[1].x, 50.0);
        assert_eq!(layout.children[1].y, 75.0);
    }

    #[test]
    fn flex_grow_shrink_at_most() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![stretch::style::Node {
                children: vec![stretch::style::Node { flex_grow: 1.0, flex_shrink: 1.0, ..Default::default() }],
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 0.0);
        assert_eq!(layout.children[0].height, 100.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[0].children[0].width, 0.0);
        assert_eq!(layout.children[0].children[0].height, 100.0);
        assert_eq!(layout.children[0].children[0].x, 0.0);
        assert_eq!(layout.children[0].children[0].y, 0.0);
    }

    #[test]
    fn percentage_padding_should_calculate_based_only_on_width() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            width: stretch::style::Dimension::Points(200.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![stretch::style::Node {
                flex_direction: stretch::style::FlexDirection::Column,
                flex_grow: 1.0,
                padding: stretch::style::Edges {
                    start: stretch::style::Dimension::Percent(0.1),
                    end: stretch::style::Dimension::Percent(0.1),
                    top: stretch::style::Dimension::Percent(0.1),
                    bottom: stretch::style::Dimension::Percent(0.1),
                    ..Default::default()
                },
                children: vec![stretch::style::Node {
                    width: stretch::style::Dimension::Points(10.0),
                    height: stretch::style::Dimension::Points(10.0),
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 200.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 200.0);
        assert_eq!(layout.children[0].height, 100.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[0].children[0].width, 10.0);
        assert_eq!(layout.children[0].children[0].height, 10.0);
        assert_eq!(layout.children[0].children[0].x, 20.0);
        assert_eq!(layout.children[0].children[0].y, 20.0);
    }

    #[test]
    fn flex_shrink_to_zero() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(75.0),
            children: vec![
                stretch::style::Node {
                    flex_shrink: 0.0,
                    width: stretch::style::Dimension::Points(50.0),
                    height: stretch::style::Dimension::Points(50.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_shrink: 1.0,
                    width: stretch::style::Dimension::Points(50.0),
                    height: stretch::style::Dimension::Points(50.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_shrink: 0.0,
                    width: stretch::style::Dimension::Points(50.0),
                    height: stretch::style::Dimension::Points(50.0),
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 75.0);
        assert_eq!(layout.height, 50.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 50.0);
        assert_eq!(layout.children[0].height, 50.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 0.0);
        assert_eq!(layout.children[1].height, 50.0);
        assert_eq!(layout.children[1].x, 50.0);
        assert_eq!(layout.children[1].y, 0.0);

        assert_eq!(layout.children[2].width, 50.0);
        assert_eq!(layout.children[2].height, 50.0);
        assert_eq!(layout.children[2].x, 50.0);
        assert_eq!(layout.children[2].y, 0.0);
    }

    #[test]
    fn margin_auto_mutiple_children_row() {
        let layout = stretch::compute(&stretch::style::Node {
            align_items: stretch::style::AlignItems::Center,
            width: stretch::style::Dimension::Points(200.0),
            height: stretch::style::Dimension::Points(200.0),
            children: vec![
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(50.0),
                    height: stretch::style::Dimension::Points(50.0),
                    margin: stretch::style::Edges { end: stretch::style::Dimension::Auto, ..Default::default() },
                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(50.0),
                    height: stretch::style::Dimension::Points(50.0),
                    margin: stretch::style::Edges { end: stretch::style::Dimension::Auto, ..Default::default() },
                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(50.0),
                    height: stretch::style::Dimension::Points(50.0),
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 200.0);
        assert_eq!(layout.height, 200.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 50.0);
        assert_eq!(layout.children[0].height, 50.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 75.0);

        assert_eq!(layout.children[1].width, 50.0);
        assert_eq!(layout.children[1].height, 50.0);
        assert_eq!(layout.children[1].x, 75.0);
        assert_eq!(layout.children[1].y, 75.0);

        assert_eq!(layout.children[2].width, 50.0);
        assert_eq!(layout.children[2].height, 50.0);
        assert_eq!(layout.children[2].x, 150.0);
        assert_eq!(layout.children[2].y, 75.0);
    }

    #[test]
    fn justify_content_min_width_with_padding_child_width_lower_than_parent() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            width: stretch::style::Dimension::Points(1080.0),
            height: stretch::style::Dimension::Points(1584.0),
            children: vec![stretch::style::Node {
                children: vec![stretch::style::Node {
                    justify_content: stretch::style::JustifyContent::Center,
                    min_width: stretch::style::Dimension::Points(400.0),
                    padding: stretch::style::Edges {
                        start: stretch::style::Dimension::Points(100.0),
                        end: stretch::style::Dimension::Points(100.0),
                        ..Default::default()
                    },
                    children: vec![stretch::style::Node {
                        width: stretch::style::Dimension::Points(199.0),
                        height: stretch::style::Dimension::Points(100.0),
                        ..Default::default()
                    }],
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 1080.0);
        assert_eq!(layout.height, 1584.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 1080.0);
        assert_eq!(layout.children[0].height, 100.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[0].children[0].width, 400.0);
        assert_eq!(layout.children[0].children[0].height, 100.0);
        assert_eq!(layout.children[0].children[0].x, 0.0);
        assert_eq!(layout.children[0].children[0].y, 0.0);

        assert_eq!(layout.children[0].children[0].children[0].width, 199.0);
        assert_eq!(layout.children[0].children[0].children[0].height, 100.0);
        assert_eq!(layout.children[0].children[0].children[0].x, 101.0);
        assert_eq!(layout.children[0].children[0].children[0].y, 0.0);
    }

    #[test]
    fn size_defined_by_grand_child() {
        let layout = stretch::compute(&stretch::style::Node {
            children: vec![stretch::style::Node {
                children: vec![stretch::style::Node {
                    width: stretch::style::Dimension::Points(100.0),
                    height: stretch::style::Dimension::Points(100.0),
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 100.0);
        assert_eq!(layout.children[0].height, 100.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[0].children[0].width, 100.0);
        assert_eq!(layout.children[0].children[0].height, 100.0);
        assert_eq!(layout.children[0].children[0].x, 0.0);
        assert_eq!(layout.children[0].children[0].y, 0.0);
    }

    #[test]
    fn absolute_layout_align_items_and_justify_content_center_and_bottom_position() {
        let layout = stretch::compute(&stretch::style::Node {
            align_items: stretch::style::AlignItems::Center,
            justify_content: stretch::style::JustifyContent::Center,
            width: stretch::style::Dimension::Points(110.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![stretch::style::Node {
                position: stretch::style::Position::Absolute,
                width: stretch::style::Dimension::Points(60.0),
                height: stretch::style::Dimension::Points(40.0),
                bottom: stretch::style::Dimension::Points(10.0),
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 110.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 60.0);
        assert_eq!(layout.children[0].height, 40.0);
        assert_eq!(layout.children[0].x, 25.0);
        assert_eq!(layout.children[0].y, 50.0);
    }

    #[test]
    fn justify_content_min_max() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            justify_content: stretch::style::JustifyContent::Center,
            width: stretch::style::Dimension::Points(100.0),
            min_height: stretch::style::Dimension::Points(100.0),
            max_height: stretch::style::Dimension::Points(200.0),
            children: vec![stretch::style::Node {
                width: stretch::style::Dimension::Points(60.0),
                height: stretch::style::Dimension::Points(60.0),
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 60.0);
        assert_eq!(layout.children[0].height, 60.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 20.0);
    }

    #[test]
    fn padding_align_end_child() {
        let layout = stretch::compute(&stretch::style::Node {
            align_items: stretch::style::AlignItems::FlexEnd,
            justify_content: stretch::style::JustifyContent::FlexEnd,
            width: stretch::style::Dimension::Points(200.0),
            height: stretch::style::Dimension::Points(200.0),
            children: vec![stretch::style::Node {
                width: stretch::style::Dimension::Points(100.0),
                height: stretch::style::Dimension::Points(100.0),
                padding: stretch::style::Edges {
                    start: stretch::style::Dimension::Points(20.0),
                    end: stretch::style::Dimension::Points(20.0),
                    top: stretch::style::Dimension::Points(20.0),
                    bottom: stretch::style::Dimension::Points(20.0),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 200.0);
        assert_eq!(layout.height, 200.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 100.0);
        assert_eq!(layout.children[0].height, 100.0);
        assert_eq!(layout.children[0].x, 100.0);
        assert_eq!(layout.children[0].y, 100.0);
    }

    #[test]
    fn justify_content_row_space_between() {
        let layout = stretch::compute(&stretch::style::Node {
            justify_content: stretch::style::JustifyContent::SpaceBetween,
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![
                stretch::style::Node { width: stretch::style::Dimension::Points(10.0), ..Default::default() },
                stretch::style::Node { width: stretch::style::Dimension::Points(10.0), ..Default::default() },
                stretch::style::Node { width: stretch::style::Dimension::Points(10.0), ..Default::default() },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 10.0);
        assert_eq!(layout.children[0].height, 100.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 10.0);
        assert_eq!(layout.children[1].height, 100.0);
        assert_eq!(layout.children[1].x, 45.0);
        assert_eq!(layout.children[1].y, 0.0);

        assert_eq!(layout.children[2].width, 10.0);
        assert_eq!(layout.children[2].height, 100.0);
        assert_eq!(layout.children[2].x, 90.0);
        assert_eq!(layout.children[2].y, 0.0);
    }

    #[test]
    fn align_self_baseline() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![
                stretch::style::Node {
                    align_self: stretch::style::AlignSelf::Baseline,
                    width: stretch::style::Dimension::Points(50.0),
                    height: stretch::style::Dimension::Points(50.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    align_self: stretch::style::AlignSelf::Baseline,
                    width: stretch::style::Dimension::Points(50.0),
                    height: stretch::style::Dimension::Points(20.0),
                    children: vec![stretch::style::Node {
                        width: stretch::style::Dimension::Points(50.0),
                        height: stretch::style::Dimension::Points(10.0),
                        ..Default::default()
                    }],
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 50.0);
        assert_eq!(layout.children[0].height, 50.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 50.0);
        assert_eq!(layout.children[1].height, 20.0);
        assert_eq!(layout.children[1].x, 50.0);
        assert_eq!(layout.children[1].y, 40.0);

        assert_eq!(layout.children[1].children[0].width, 50.0);
        assert_eq!(layout.children[1].children[0].height, 10.0);
        assert_eq!(layout.children[1].children[0].x, 0.0);
        assert_eq!(layout.children[1].children[0].y, 0.0);
    }

    #[test]
    fn rounding_fractial_input_1() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(113.4),
            children: vec![
                stretch::style::Node {
                    flex_grow: 1.0,
                    flex_basis: stretch::style::Dimension::Points(50.0),
                    height: stretch::style::Dimension::Points(20.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 1.0,
                    height: stretch::style::Dimension::Points(10.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 1.0,
                    height: stretch::style::Dimension::Points(10.0),
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 113.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 100.0);
        assert_eq!(layout.children[0].height, 64.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 100.0);
        assert_eq!(layout.children[1].height, 25.0);
        assert_eq!(layout.children[1].x, 0.0);
        assert_eq!(layout.children[1].y, 64.0);

        assert_eq!(layout.children[2].width, 100.0);
        assert_eq!(layout.children[2].height, 24.0);
        assert_eq!(layout.children[2].x, 0.0);
        assert_eq!(layout.children[2].y, 89.0);
    }

    #[test]
    fn absolute_layout_align_items_center() {
        let layout = stretch::compute(&stretch::style::Node {
            align_items: stretch::style::AlignItems::Center,
            width: stretch::style::Dimension::Points(110.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![stretch::style::Node {
                position: stretch::style::Position::Absolute,
                width: stretch::style::Dimension::Points(60.0),
                height: stretch::style::Dimension::Points(40.0),
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 110.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 60.0);
        assert_eq!(layout.children[0].height, 40.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 30.0);
    }

    #[test]
    fn percentage_flex_basis_cross_min_width() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            width: stretch::style::Dimension::Points(200.0),
            height: stretch::style::Dimension::Points(200.0),
            children: vec![
                stretch::style::Node {
                    flex_grow: 1.0,
                    flex_basis: stretch::style::Dimension::Percent(0.1),
                    min_width: stretch::style::Dimension::Percent(0.6),
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 4.0,
                    flex_basis: stretch::style::Dimension::Percent(0.2),
                    min_width: stretch::style::Dimension::Percent(0.2),
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 200.0);
        assert_eq!(layout.height, 200.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 200.0);
        assert_eq!(layout.children[0].height, 50.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 200.0);
        assert_eq!(layout.children[1].height, 150.0);
        assert_eq!(layout.children[1].x, 0.0);
        assert_eq!(layout.children[1].y, 50.0);
    }

    #[test]
    fn margin_auto_left_and_right_column_and_center() {
        let layout = stretch::compute(&stretch::style::Node {
            align_items: stretch::style::AlignItems::Center,
            width: stretch::style::Dimension::Points(200.0),
            height: stretch::style::Dimension::Points(200.0),
            children: vec![
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(50.0),
                    height: stretch::style::Dimension::Points(50.0),
                    margin: stretch::style::Edges {
                        start: stretch::style::Dimension::Auto,
                        end: stretch::style::Dimension::Auto,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(50.0),
                    height: stretch::style::Dimension::Points(50.0),
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 200.0);
        assert_eq!(layout.height, 200.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 50.0);
        assert_eq!(layout.children[0].height, 50.0);
        assert_eq!(layout.children[0].x, 50.0);
        assert_eq!(layout.children[0].y, 75.0);

        assert_eq!(layout.children[1].width, 50.0);
        assert_eq!(layout.children[1].height, 50.0);
        assert_eq!(layout.children[1].x, 150.0);
        assert_eq!(layout.children[1].y, 75.0);
    }

    #[test]
    fn flex_grow_less_than_factor_one() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(500.0),
            height: stretch::style::Dimension::Points(200.0),
            children: vec![
                stretch::style::Node {
                    flex_grow: 0.2,
                    flex_shrink: 0.0,
                    flex_basis: stretch::style::Dimension::Points(40.0),
                    ..Default::default()
                },
                stretch::style::Node { flex_grow: 0.2, flex_shrink: 0.0, ..Default::default() },
                stretch::style::Node { flex_grow: 0.4, flex_shrink: 0.0, ..Default::default() },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 500.0);
        assert_eq!(layout.height, 200.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 132.0);
        assert_eq!(layout.children[0].height, 200.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 92.0);
        assert_eq!(layout.children[1].height, 200.0);
        assert_eq!(layout.children[1].x, 132.0);
        assert_eq!(layout.children[1].y, 0.0);

        assert_eq!(layout.children[2].width, 184.0);
        assert_eq!(layout.children[2].height, 200.0);
        assert_eq!(layout.children[2].x, 224.0);
        assert_eq!(layout.children[2].y, 0.0);
    }

    #[test]
    fn flex_grow_within_constrained_min_max_column() {
        let layout = stretch::compute(&stretch::style::Node {
            min_height: stretch::style::Dimension::Points(100.0),
            children: vec![
                stretch::style::Node { flex_grow: 1.0, ..Default::default() },
                stretch::style::Node { height: stretch::style::Dimension::Points(50.0), ..Default::default() },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 0.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 0.0);
        assert_eq!(layout.children[0].height, 100.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 0.0);
        assert_eq!(layout.children[1].height, 50.0);
        assert_eq!(layout.children[1].x, 0.0);
        assert_eq!(layout.children[1].y, 0.0);
    }

    #[test]
    fn percentage_flex_basis_cross_max_height() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            width: stretch::style::Dimension::Points(200.0),
            height: stretch::style::Dimension::Points(200.0),
            children: vec![
                stretch::style::Node {
                    flex_grow: 1.0,
                    flex_basis: stretch::style::Dimension::Percent(0.1),
                    max_height: stretch::style::Dimension::Percent(0.6),
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 4.0,
                    flex_basis: stretch::style::Dimension::Percent(0.1),
                    max_height: stretch::style::Dimension::Percent(0.2),
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 200.0);
        assert_eq!(layout.height, 200.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 200.0);
        assert_eq!(layout.children[0].height, 120.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 200.0);
        assert_eq!(layout.children[1].height, 40.0);
        assert_eq!(layout.children[1].x, 0.0);
        assert_eq!(layout.children[1].y, 120.0);
    }

    #[test]
    fn justify_content_column_flex_end() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            justify_content: stretch::style::JustifyContent::FlexEnd,
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![
                stretch::style::Node { height: stretch::style::Dimension::Points(10.0), ..Default::default() },
                stretch::style::Node { height: stretch::style::Dimension::Points(10.0), ..Default::default() },
                stretch::style::Node { height: stretch::style::Dimension::Points(10.0), ..Default::default() },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 100.0);
        assert_eq!(layout.children[0].height, 10.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 70.0);

        assert_eq!(layout.children[1].width, 100.0);
        assert_eq!(layout.children[1].height, 10.0);
        assert_eq!(layout.children[1].x, 0.0);
        assert_eq!(layout.children[1].y, 80.0);

        assert_eq!(layout.children[2].width, 100.0);
        assert_eq!(layout.children[2].height, 10.0);
        assert_eq!(layout.children[2].x, 0.0);
        assert_eq!(layout.children[2].y, 90.0);
    }

    #[test]
    fn rounding_flex_basis_flex_grow_row_width_of_100() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![
                stretch::style::Node { flex_grow: 1.0, ..Default::default() },
                stretch::style::Node { flex_grow: 1.0, ..Default::default() },
                stretch::style::Node { flex_grow: 1.0, ..Default::default() },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 33.0);
        assert_eq!(layout.children[0].height, 100.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 34.0);
        assert_eq!(layout.children[1].height, 100.0);
        assert_eq!(layout.children[1].x, 33.0);
        assert_eq!(layout.children[1].y, 0.0);

        assert_eq!(layout.children[2].width, 33.0);
        assert_eq!(layout.children[2].height, 100.0);
        assert_eq!(layout.children[2].x, 67.0);
        assert_eq!(layout.children[2].y, 0.0);
    }

    #[test]
    fn wrap_reverse_row_align_content_flex_start() {
        let layout = stretch::compute(&stretch::style::Node {
            flexWrap: stretch::style::FlexWrap::WrapReverse,
            align_content: stretch::style::AlignContent::FlexStart,
            width: stretch::style::Dimension::Points(100.0),
            children: vec![
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(30.0),
                    height: stretch::style::Dimension::Points(10.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(30.0),
                    height: stretch::style::Dimension::Points(20.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(30.0),
                    height: stretch::style::Dimension::Points(30.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(30.0),
                    height: stretch::style::Dimension::Points(40.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(30.0),
                    height: stretch::style::Dimension::Points(50.0),
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 80.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 30.0);
        assert_eq!(layout.children[0].height, 10.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 70.0);

        assert_eq!(layout.children[1].width, 30.0);
        assert_eq!(layout.children[1].height, 20.0);
        assert_eq!(layout.children[1].x, 30.0);
        assert_eq!(layout.children[1].y, 60.0);

        assert_eq!(layout.children[2].width, 30.0);
        assert_eq!(layout.children[2].height, 30.0);
        assert_eq!(layout.children[2].x, 60.0);
        assert_eq!(layout.children[2].y, 50.0);

        assert_eq!(layout.children[3].width, 30.0);
        assert_eq!(layout.children[3].height, 40.0);
        assert_eq!(layout.children[3].x, 0.0);
        assert_eq!(layout.children[3].y, 10.0);

        assert_eq!(layout.children[4].width, 30.0);
        assert_eq!(layout.children[4].height, 50.0);
        assert_eq!(layout.children[4].x, 30.0);
        assert_eq!(layout.children[4].y, 0.0);
    }

    #[test]
    fn flex_shrink_flex_grow_child_flex_shrink_other_child() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(500.0),
            height: stretch::style::Dimension::Points(500.0),
            children: vec![
                stretch::style::Node {
                    flex_grow: 0.0,
                    flex_shrink: 1.0,
                    width: stretch::style::Dimension::Points(500.0),
                    height: stretch::style::Dimension::Points(100.0),
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 1.0,
                    flex_shrink: 1.0,
                    width: stretch::style::Dimension::Points(500.0),
                    height: stretch::style::Dimension::Points(100.0),
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 500.0);
        assert_eq!(layout.height, 500.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 250.0);
        assert_eq!(layout.children[0].height, 100.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 250.0);
        assert_eq!(layout.children[1].height, 100.0);
        assert_eq!(layout.children[1].x, 250.0);
        assert_eq!(layout.children[1].y, 0.0);
    }

    #[test]
    fn align_items_flex_end() {
        let layout = stretch::compute(&stretch::style::Node {
            align_items: stretch::style::AlignItems::FlexEnd,
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![stretch::style::Node {
                width: stretch::style::Dimension::Points(10.0),
                height: stretch::style::Dimension::Points(10.0),
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 10.0);
        assert_eq!(layout.children[0].height, 10.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 90.0);
    }

    #[test]
    fn absolute_layout_width_height_start_top() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![stretch::style::Node {
                position: stretch::style::Position::Absolute,
                width: stretch::style::Dimension::Points(10.0),
                height: stretch::style::Dimension::Points(10.0),
                start: stretch::style::Dimension::Points(10.0),
                top: stretch::style::Dimension::Points(10.0),
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 10.0);
        assert_eq!(layout.children[0].height, 10.0);
        assert_eq!(layout.children[0].x, 10.0);
        assert_eq!(layout.children[0].y, 10.0);
    }

    #[test]
    fn rounding_flex_basis_flex_shrink_row() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(101.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![
                stretch::style::Node {
                    flex_shrink: 1.0,
                    flex_basis: stretch::style::Dimension::Points(100.0),
                    ..Default::default()
                },
                stretch::style::Node { flex_basis: stretch::style::Dimension::Points(25.0), ..Default::default() },
                stretch::style::Node { flex_basis: stretch::style::Dimension::Points(25.0), ..Default::default() },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 101.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 67.0);
        assert_eq!(layout.children[0].height, 100.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 17.0);
        assert_eq!(layout.children[1].height, 100.0);
        assert_eq!(layout.children[1].x, 67.0);
        assert_eq!(layout.children[1].y, 0.0);

        assert_eq!(layout.children[2].width, 17.0);
        assert_eq!(layout.children[2].height, 100.0);
        assert_eq!(layout.children[2].x, 84.0);
        assert_eq!(layout.children[2].y, 0.0);
    }

    #[test]
    fn border_center_child() {
        let layout = stretch::compute(&stretch::style::Node {
            align_items: stretch::style::AlignItems::Center,
            justify_content: stretch::style::JustifyContent::Center,
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            border: stretch::style::Edges {
                top: stretch::style::Dimension::Points(10.0),
                bottom: stretch::style::Dimension::Points(20.0),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                width: stretch::style::Dimension::Points(10.0),
                height: stretch::style::Dimension::Points(10.0),
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 10.0);
        assert_eq!(layout.children[0].height, 10.0);
        assert_eq!(layout.children[0].x, 45.0);
        assert_eq!(layout.children[0].y, 40.0);
    }

    #[test]
    fn align_items_flex_start() {
        let layout = stretch::compute(&stretch::style::Node {
            align_items: stretch::style::AlignItems::FlexStart,
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![stretch::style::Node {
                width: stretch::style::Dimension::Points(10.0),
                height: stretch::style::Dimension::Points(10.0),
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 10.0);
        assert_eq!(layout.children[0].height, 10.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);
    }

    #[test]
    fn border_no_child() {
        let layout = stretch::compute(&stretch::style::Node {
            border: stretch::style::Edges {
                start: stretch::style::Dimension::Points(10.0),
                end: stretch::style::Dimension::Points(10.0),
                top: stretch::style::Dimension::Points(10.0),
                bottom: stretch::style::Dimension::Points(10.0),
                ..Default::default()
            },
            ..Default::default()
        });

        assert_eq!(layout.width, 20.0);
        assert_eq!(layout.height, 20.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);
    }

    #[test]
    fn rounding_flex_basis_flex_grow_row_prime_number_width() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(113.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![
                stretch::style::Node { flex_grow: 1.0, ..Default::default() },
                stretch::style::Node { flex_grow: 1.0, ..Default::default() },
                stretch::style::Node { flex_grow: 1.0, ..Default::default() },
                stretch::style::Node { flex_grow: 1.0, ..Default::default() },
                stretch::style::Node { flex_grow: 1.0, ..Default::default() },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 113.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 23.0);
        assert_eq!(layout.children[0].height, 100.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 22.0);
        assert_eq!(layout.children[1].height, 100.0);
        assert_eq!(layout.children[1].x, 23.0);
        assert_eq!(layout.children[1].y, 0.0);

        assert_eq!(layout.children[2].width, 23.0);
        assert_eq!(layout.children[2].height, 100.0);
        assert_eq!(layout.children[2].x, 45.0);
        assert_eq!(layout.children[2].y, 0.0);

        assert_eq!(layout.children[3].width, 22.0);
        assert_eq!(layout.children[3].height, 100.0);
        assert_eq!(layout.children[3].x, 68.0);
        assert_eq!(layout.children[3].y, 0.0);

        assert_eq!(layout.children[4].width, 23.0);
        assert_eq!(layout.children[4].height, 100.0);
        assert_eq!(layout.children[4].x, 90.0);
        assert_eq!(layout.children[4].y, 0.0);
    }

    #[test]
    fn align_items_center_with_child_top() {
        let layout = stretch::compute(&stretch::style::Node {
            align_items: stretch::style::AlignItems::Center,
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![stretch::style::Node {
                width: stretch::style::Dimension::Points(10.0),
                height: stretch::style::Dimension::Points(10.0),
                top: stretch::style::Dimension::Points(10.0),
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 10.0);
        assert_eq!(layout.children[0].height, 10.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 55.0);
    }

    #[test]
    fn percentage_width_height_undefined_parent_size() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            children: vec![stretch::style::Node {
                width: stretch::style::Dimension::Percent(0.5),
                height: stretch::style::Dimension::Percent(0.5),
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 0.0);
        assert_eq!(layout.height, 0.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 0.0);
        assert_eq!(layout.children[0].height, 0.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);
    }

    #[test]
    fn flex_grow_within_max_width() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            width: stretch::style::Dimension::Points(200.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![stretch::style::Node {
                max_width: stretch::style::Dimension::Points(100.0),
                children: vec![stretch::style::Node {
                    flex_grow: 1.0,
                    height: stretch::style::Dimension::Points(20.0),
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 200.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 100.0);
        assert_eq!(layout.children[0].height, 20.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[0].children[0].width, 100.0);
        assert_eq!(layout.children[0].children[0].height, 20.0);
        assert_eq!(layout.children[0].children[0].x, 0.0);
        assert_eq!(layout.children[0].children[0].y, 0.0);
    }

    #[test]
    fn flex_grow_height_maximized() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(500.0),
            children: vec![stretch::style::Node {
                flex_direction: stretch::style::FlexDirection::Column,
                flex_grow: 1.0,
                min_height: stretch::style::Dimension::Points(100.0),
                max_height: stretch::style::Dimension::Points(500.0),
                children: vec![
                    stretch::style::Node {
                        flex_grow: 1.0,
                        flex_basis: stretch::style::Dimension::Points(200.0),
                        ..Default::default()
                    },
                    stretch::style::Node { height: stretch::style::Dimension::Points(100.0), ..Default::default() },
                ],
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 500.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 100.0);
        assert_eq!(layout.children[0].height, 500.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[0].children[0].width, 100.0);
        assert_eq!(layout.children[0].children[0].height, 400.0);
        assert_eq!(layout.children[0].children[0].x, 0.0);
        assert_eq!(layout.children[0].children[0].y, 0.0);

        assert_eq!(layout.children[0].children[1].width, 100.0);
        assert_eq!(layout.children[0].children[1].height, 100.0);
        assert_eq!(layout.children[0].children[1].x, 0.0);
        assert_eq!(layout.children[0].children[1].y, 400.0);
    }

    #[test]
    fn max_height_overrides_height() {
        let layout = stretch::compute(&stretch::style::Node {
            children: vec![stretch::style::Node {
                height: stretch::style::Dimension::Points(200.0),
                max_height: stretch::style::Dimension::Points(100.0),
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 0.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 0.0);
        assert_eq!(layout.children[0].height, 100.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);
    }

    #[test]
    fn justify_content_min_width_with_padding_child_width_greater_than_parent() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            width: stretch::style::Dimension::Points(1000.0),
            height: stretch::style::Dimension::Points(1584.0),
            children: vec![stretch::style::Node {
                children: vec![stretch::style::Node {
                    justify_content: stretch::style::JustifyContent::Center,
                    min_width: stretch::style::Dimension::Points(400.0),
                    padding: stretch::style::Edges {
                        start: stretch::style::Dimension::Points(100.0),
                        end: stretch::style::Dimension::Points(100.0),
                        ..Default::default()
                    },
                    children: vec![stretch::style::Node {
                        width: stretch::style::Dimension::Points(300.0),
                        height: stretch::style::Dimension::Points(100.0),
                        ..Default::default()
                    }],
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 1000.0);
        assert_eq!(layout.height, 1584.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 1000.0);
        assert_eq!(layout.children[0].height, 100.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[0].children[0].width, 500.0);
        assert_eq!(layout.children[0].children[0].height, 100.0);
        assert_eq!(layout.children[0].children[0].x, 0.0);
        assert_eq!(layout.children[0].children[0].y, 0.0);

        assert_eq!(layout.children[0].children[0].children[0].width, 300.0);
        assert_eq!(layout.children[0].children[0].children[0].height, 100.0);
        assert_eq!(layout.children[0].children[0].children[0].x, 100.0);
        assert_eq!(layout.children[0].children[0].children[0].y, 0.0);
    }

    #[test]
    fn percent_absolute_position() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            width: stretch::style::Dimension::Points(60.0),
            height: stretch::style::Dimension::Points(50.0),
            children: vec![stretch::style::Node {
                position: stretch::style::Position::Absolute,
                width: stretch::style::Dimension::Percent(1.0),
                height: stretch::style::Dimension::Points(50.0),
                start: stretch::style::Dimension::Percent(0.5),
                children: vec![
                    stretch::style::Node { width: stretch::style::Dimension::Percent(1.0), ..Default::default() },
                    stretch::style::Node { width: stretch::style::Dimension::Percent(1.0), ..Default::default() },
                ],
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 60.0);
        assert_eq!(layout.height, 50.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 60.0);
        assert_eq!(layout.children[0].height, 50.0);
        assert_eq!(layout.children[0].x, 30.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[0].children[0].width, 30.0);
        assert_eq!(layout.children[0].children[0].height, 50.0);
        assert_eq!(layout.children[0].children[0].x, 0.0);
        assert_eq!(layout.children[0].children[0].y, 0.0);

        assert_eq!(layout.children[0].children[1].width, 30.0);
        assert_eq!(layout.children[0].children[1].height, 50.0);
        assert_eq!(layout.children[0].children[1].x, 30.0);
        assert_eq!(layout.children[0].children[1].y, 0.0);
    }

    #[test]
    fn flex_root_ignored() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            width: stretch::style::Dimension::Points(100.0),
            min_height: stretch::style::Dimension::Points(100.0),
            max_height: stretch::style::Dimension::Points(500.0),
            children: vec![
                stretch::style::Node {
                    flex_grow: 1.0,
                    flex_basis: stretch::style::Dimension::Points(200.0),
                    ..Default::default()
                },
                stretch::style::Node { height: stretch::style::Dimension::Points(100.0), ..Default::default() },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 300.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 100.0);
        assert_eq!(layout.children[0].height, 200.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 100.0);
        assert_eq!(layout.children[1].height, 100.0);
        assert_eq!(layout.children[1].x, 0.0);
        assert_eq!(layout.children[1].y, 200.0);
    }

    #[test]
    fn justify_content_column_space_between() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            justify_content: stretch::style::JustifyContent::SpaceBetween,
            width: stretch::style::Dimension::Points(100.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![
                stretch::style::Node { height: stretch::style::Dimension::Points(10.0), ..Default::default() },
                stretch::style::Node { height: stretch::style::Dimension::Points(10.0), ..Default::default() },
                stretch::style::Node { height: stretch::style::Dimension::Points(10.0), ..Default::default() },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 100.0);
        assert_eq!(layout.children[0].height, 10.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[1].width, 100.0);
        assert_eq!(layout.children[1].height, 10.0);
        assert_eq!(layout.children[1].x, 0.0);
        assert_eq!(layout.children[1].y, 45.0);

        assert_eq!(layout.children[2].width, 100.0);
        assert_eq!(layout.children[2].height, 10.0);
        assert_eq!(layout.children[2].x, 0.0);
        assert_eq!(layout.children[2].y, 90.0);
    }

    #[test]
    fn flex_grow_within_constrained_max_width() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            width: stretch::style::Dimension::Points(200.0),
            height: stretch::style::Dimension::Points(100.0),
            children: vec![stretch::style::Node {
                max_width: stretch::style::Dimension::Points(300.0),
                children: vec![stretch::style::Node {
                    flex_grow: 1.0,
                    height: stretch::style::Dimension::Points(20.0),
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 200.0);
        assert_eq!(layout.height, 100.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 200.0);
        assert_eq!(layout.children[0].height, 20.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[0].children[0].width, 200.0);
        assert_eq!(layout.children[0].children[0].height, 20.0);
        assert_eq!(layout.children[0].children[0].x, 0.0);
        assert_eq!(layout.children[0].children[0].y, 0.0);
    }

    #[test]
    fn max_width_overrides_width() {
        let layout = stretch::compute(&stretch::style::Node {
            children: vec![stretch::style::Node {
                width: stretch::style::Dimension::Points(200.0),
                max_width: stretch::style::Dimension::Points(100.0),
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 100.0);
        assert_eq!(layout.height, 0.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 100.0);
        assert_eq!(layout.children[0].height, 0.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);
    }

    #[test]
    fn percentage_position_bottom_right() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(500.0),
            height: stretch::style::Dimension::Points(500.0),
            children: vec![stretch::style::Node {
                width: stretch::style::Dimension::Percent(0.6),
                height: stretch::style::Dimension::Percent(0.2),
                end: stretch::style::Dimension::Percent(0.2),
                bottom: stretch::style::Dimension::Percent(0.1),
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 500.0);
        assert_eq!(layout.height, 500.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 275.0);
        assert_eq!(layout.children[0].height, 75.0);
        assert_eq!(layout.children[0].x, -100.0);
        assert_eq!(layout.children[0].y, -50.0);
    }

    #[test]
    fn margin_auto_top_stretching_child() {
        let layout = stretch::compute(&stretch::style::Node {
            align_items: stretch::style::AlignItems::Center,
            width: stretch::style::Dimension::Points(200.0),
            height: stretch::style::Dimension::Points(200.0),
            children: vec![
                stretch::style::Node {
                    flex_grow: 1.0,
                    flex_shrink: 1.0,
                    flex_basis: stretch::style::Dimension::Percent(0.0),
                    margin: stretch::style::Edges { top: stretch::style::Dimension::Auto, ..Default::default() },
                    ..Default::default()
                },
                stretch::style::Node {
                    width: stretch::style::Dimension::Points(50.0),
                    height: stretch::style::Dimension::Points(50.0),
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        assert_eq!(layout.width, 200.0);
        assert_eq!(layout.height, 200.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 150.0);
        assert_eq!(layout.children[0].height, 0.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 200.0);

        assert_eq!(layout.children[1].width, 50.0);
        assert_eq!(layout.children[1].height, 50.0);
        assert_eq!(layout.children[1].x, 150.0);
        assert_eq!(layout.children[1].y, 75.0);
    }

    #[test]
    fn justify_content_column_min_height_and_margin() {
        let layout = stretch::compute(&stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            justify_content: stretch::style::JustifyContent::Center,
            min_height: stretch::style::Dimension::Points(50.0),
            margin: stretch::style::Edges { top: stretch::style::Dimension::Points(100.0), ..Default::default() },
            children: vec![stretch::style::Node {
                width: stretch::style::Dimension::Points(20.0),
                height: stretch::style::Dimension::Points(20.0),
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 20.0);
        assert_eq!(layout.height, 50.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 100.0);

        assert_eq!(layout.children[0].width, 20.0);
        assert_eq!(layout.children[0].height, 20.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 15.0);
    }

    #[test]
    fn align_flex_start_with_shrinking_children() {
        let layout = stretch::compute(&stretch::style::Node {
            width: stretch::style::Dimension::Points(500.0),
            height: stretch::style::Dimension::Points(500.0),
            children: vec![stretch::style::Node {
                align_items: stretch::style::AlignItems::FlexStart,
                children: vec![stretch::style::Node {
                    flex_grow: 1.0,
                    flex_shrink: 1.0,
                    children: vec![stretch::style::Node { flex_grow: 1.0, flex_shrink: 1.0, ..Default::default() }],
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        });

        assert_eq!(layout.width, 500.0);
        assert_eq!(layout.height, 500.0);
        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, 0.0);

        assert_eq!(layout.children[0].width, 0.0);
        assert_eq!(layout.children[0].height, 500.0);
        assert_eq!(layout.children[0].x, 0.0);
        assert_eq!(layout.children[0].y, 0.0);

        assert_eq!(layout.children[0].children[0].width, 0.0);
        assert_eq!(layout.children[0].children[0].height, 0.0);
        assert_eq!(layout.children[0].children[0].x, 0.0);
        assert_eq!(layout.children[0].children[0].y, 0.0);

        assert_eq!(layout.children[0].children[0].children[0].width, 0.0);
        assert_eq!(layout.children[0].children[0].children[0].height, 0.0);
        assert_eq!(layout.children[0].children[0].children[0].x, 0.0);
        assert_eq!(layout.children[0].children[0].children[0].y, 0.0);
    }

}
