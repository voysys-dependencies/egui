use egui::{
    pos2, vec2, Area, CentralPanel, Context, Event, Id, Order, RawInput, Rect, Sense, SidePanel,
    TopBottomPanel,
};

fn layout(ctx: &Context, central: bool) {
    TopBottomPanel::top("top")
        .exact_height(30.0)
        .show(ctx, |ui| {
            ui.label("top");
        });
    SidePanel::left("left").exact_width(80.0).show(ctx, |ui| {
        ui.label("left");
    });
    Area::new(Id::new("window"))
        .fixed_pos(pos2(150.0, 100.0))
        .show(ctx, |ui| {
            ui.allocate_exact_size(vec2(80.0, 60.0), Sense::click());
        });
    Area::new(Id::new("decoration"))
        .order(Order::Tooltip)
        .interactable(false)
        .fixed_pos(pos2(300.0, 100.0))
        .show(ctx, |ui| {
            ui.allocate_exact_size(vec2(60.0, 60.0), Sense::hover());
        });
    if central {
        CentralPanel::default().show(ctx, |ui| {
            ui.label("center");
        });
    }
}

#[test]
fn regions_match_hit_testing_for_windows_panels_and_unused_space() {
    for central in [false, true] {
        let ctx = Context::default();
        for _ in 0..3 {
            let _ = ctx.run(
                RawInput {
                    screen_rect: Some(Rect::from_min_size(pos2(0.0, 0.0), vec2(500.0, 400.0))),
                    ..Default::default()
                },
                |ctx| layout(ctx, central),
            );
        }
        for point in [
            pos2(10.0, 10.0),
            pos2(40.0, 150.0),
            pos2(180.0, 130.0),
            pos2(320.0, 130.0),
            pos2(450.0, 300.0),
        ] {
            let _ = ctx.run(
                RawInput {
                    events: vec![Event::PointerMoved(point)],
                    ..Default::default()
                },
                |ctx| layout(ctx, central),
            );
            assert_eq!(
                ctx.pointer_input_regions()
                    .iter()
                    .any(|r| r.contains(point)),
                ctx.is_pointer_over_area(),
                "central={central}, point={point:?}"
            );
        }
        if !central {
            let regions = ctx.pointer_input_regions();
            assert!(!regions.iter().any(|r| r.contains(pos2(450.0, 300.0))));
            assert!(!regions.iter().any(|r| r.contains(pos2(320.0, 130.0))));
        }
    }
}
