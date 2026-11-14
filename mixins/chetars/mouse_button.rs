fn mouse_button(
    game: &mut crate::game::Game<'_>,
    button: i32,
    pressed: bool,
    modifiers: u32,
    canceled: &mut bool,
) {
    let _ = modifiers;

    // Always release the drag latch, even when gameplay was just hidden by
    // another screen or the overlay was closed while RMB remained held.
    if button == 1 && !pressed {
        crate::game::chetars_spatial_gui::set_dragging(false);
        if !game.native_playing() || !crate::game::chetars_spatial_gui::is_open() {
            return;
        }
        *canceled = true;
        return;
    }

    if !game.native_playing() || !crate::game::chetars_spatial_gui::is_open() {
        return;
    }

    // GLFW button 1 is the right mouse button. While held, camera rotation
    // is converted into panel movement by world_submit.rs.
    if button == 1 {
        crate::game::chetars_spatial_gui::set_dragging(true);
        *canceled = true;
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
