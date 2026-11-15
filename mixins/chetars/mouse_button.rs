fn mouse_button(
    game: &mut crate::game::Game<'_>,
    button: i32,
    pressed: bool,
    modifiers: u32,
    canceled: &mut bool,
) {
    let _ = modifiers;

    // Release only a drag that this panel actually captured.
    if button == 1 && !pressed {
        let was_dragging = crate::game::chetars_spatial_gui::is_dragging();
        crate::game::chetars_spatial_gui::set_dragging(false);
        if was_dragging {
            *canceled = true;
        }
        return;
    }

    if !game.native_playing() || !crate::game::chetars_spatial_gui::is_open() {
        return;
    }

    // GLFW button 1 is RMB. It starts a drag only when the crosshair is over
    // the full-width title bar; elsewhere Minecraft keeps its normal use action.
    if button == 1 && pressed {
        if crate::game::chetars_spatial_gui::can_drag_from_header() {
            crate::game::chetars_spatial_gui::set_dragging(true);
            *canceled = true;
        }
        return;
    }

    // Left-click the settings row under the center crosshair to change mode.
    if button == 0
        && pressed
        && crate::game::chetars_spatial_gui::selected_button() == 2
    {
        if let Some(index) = crate::game::chetars_spatial_gui::hovered_setting() {
            crate::game::chetars_spatial_gui::select_setting(index);
            crate::game::chetars_spatial_gui::set_follow_mode(index == 1);
            *canceled = true;
        }
    }
}
