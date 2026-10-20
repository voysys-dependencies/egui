use egui::{
    pos2, vec2, CentralPanel, Context, Event, Id, Modifiers, PointerButton, RawInput, Rect, Sense,
};

fn button(pos: egui::Pos2, pressed: bool) -> Event {
    Event::PointerButton {
        pos,
        button: PointerButton::Primary,
        pressed,
        modifiers: Modifiers::default(),
    }
}

fn frame(
    ctx: &Context,
    time: f64,
    events: Vec<Event>,
    sense: Sense,
    enabled: bool,
    clip: bool,
    parent: bool,
) -> (bool, bool) {
    let mut result = (false, false);
    let _ = ctx.run(
        RawInput {
            screen_rect: Some(Rect::from_min_size(pos2(0.0, 0.0), vec2(500.0, 300.0))),
            time: Some(time),
            events,
            ..Default::default()
        },
        |ctx| {
            CentralPanel::default().show(ctx, |ui| {
                if parent {
                    ui.interact(
                        Rect::from_min_max(pos2(10.0, 10.0), pos2(400.0, 250.0)),
                        Id::new("parent"),
                        Sense::click_and_drag(),
                    );
                }
                ui.set_enabled(enabled);
                if clip {
                    ui.set_clip_rect(Rect::from_min_max(pos2(10.0, 10.0), pos2(60.0, 100.0)));
                }
                let response = ui.interact(
                    Rect::from_min_max(pos2(20.0, 20.0), pos2(200.0, 100.0)),
                    Id::new("button"),
                    sense,
                );
                result = (response.clicked(), response.dragged());
                ui.interact(
                    Rect::from_min_max(pos2(220.0, 20.0), pos2(400.0, 100.0)),
                    Id::new("other"),
                    Sense::click(),
                );
            });
        },
    );
    result
}

fn gesture(
    policy: bool,
    end: egui::Pos2,
    sense: Sense,
    enabled: bool,
    clip: bool,
    gone: bool,
    parent: bool,
) -> (bool, bool) {
    let ctx = Context::default();
    ctx.style_mut(|s| s.interaction.click_on_release = policy);
    frame(&ctx, 0.0, vec![], sense, enabled, clip, parent);
    let start = pos2(40.0, 50.0);
    frame(
        &ctx,
        0.1,
        vec![Event::PointerMoved(start), button(start, true)],
        sense,
        enabled,
        clip,
        parent,
    );
    let held = frame(
        &ctx,
        2.0,
        vec![Event::PointerMoved(end)],
        sense,
        enabled,
        clip,
        parent,
    );
    assert_eq!(ctx.input(|i| i.pointer.latest_pos()), Some(end));
    if gone {
        frame(
            &ctx,
            2.1,
            vec![Event::PointerGone],
            sense,
            enabled,
            clip,
            parent,
        );
    }
    let released = frame(
        &ctx,
        2.2,
        vec![button(end, false)],
        sense,
        enabled,
        clip,
        parent,
    );
    (released.0, held.1)
}

#[test]
fn tracked_pointer_clicks_after_motion_and_long_hold() {
    assert!(
        gesture(
            true,
            pos2(150.0, 50.0),
            Sense::click(),
            true,
            false,
            false,
            false
        )
        .0
    );
}
#[test]
fn desktop_keeps_motion_threshold() {
    assert!(
        !gesture(
            false,
            pos2(150.0, 50.0),
            Sense::click(),
            true,
            false,
            false,
            false
        )
        .0
    );
}
#[test]
fn release_outside_or_over_another_widget_cancels() {
    for end in [pos2(210.0, 50.0), pos2(250.0, 50.0)] {
        assert!(!gesture(true, end, Sense::click(), true, false, false, false).0);
    }
}
#[test]
fn disabled_clipped_and_lost_pointer_cancel() {
    for (enabled, clip, gone) in [
        (false, false, false),
        (true, true, false),
        (true, false, true),
    ] {
        assert!(
            !gesture(
                true,
                pos2(150.0, 50.0),
                Sense::click(),
                enabled,
                clip,
                gone,
                false
            )
            .0
        );
    }
}
#[test]
fn drag_controls_keep_dragging() {
    let (clicked, dragged) = gesture(
        true,
        pos2(150.0, 50.0),
        Sense::click_and_drag(),
        true,
        false,
        false,
        false,
    );
    assert!(!clicked);
    assert!(dragged);
}
#[test]
fn dragging_parent_cancels_child_click() {
    assert!(
        !gesture(
            true,
            pos2(150.0, 50.0),
            Sense::click(),
            true,
            false,
            false,
            true
        )
        .0
    );
}
#[test]
fn policy_is_opt_in() {
    assert!(!egui::style::Interaction::default().click_on_release);
}
