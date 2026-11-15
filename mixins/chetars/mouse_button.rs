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

    // Start orbit dragging only from the top title bar, preserving RMB block use
    // everywhere else in the world.
    if button == 1 && pressed {
        if crate::game::chetars_spatial_gui::can_drag_from_header() {
            crate::game::chetars_spatial_gui::set_dragging(true);
            *canceled = true;
        }
        return;
    }

    if button != 0 || !pressed {
        return;
    }

    // Sidebar tabs were previously visual only. They now change the active
    // catalogue page when the crosshair is aimed at a tab.
    if let Some(index) = crate::game::chetars_spatial_gui::hovered_navigation() {
        crate::game::chetars_spatial_gui::select_button(index);
        *canceled = true;
        return;
    }

    if crate::game::chetars_spatial_gui::selected_button() == 2 {
        if let Some(index) = crate::game::chetars_spatial_gui::hovered_setting() {
            crate::game::chetars_spatial_gui::select_setting(index);
            crate::game::chetars_spatial_gui::set_follow_mode(index == 1);
            *canceled = true;
            return;
        }

        if let Some(index) = crate::game::chetars_spatial_gui::hovered_distance_control() {
            crate::game::chetars_spatial_gui::adjust_panel_distance(if index == 0 { -0.25 } else { 0.25 });
            *canceled = true;
        }
    }
}
