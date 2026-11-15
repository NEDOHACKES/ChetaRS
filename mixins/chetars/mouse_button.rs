fn mouse_button(
    game: &mut crate::game::Game<'_>,
    button: i32,
    pressed: bool,
    modifiers: u32,
    canceled: &mut bool,
) {
    let _ = modifiers;

    // A captured RMB drag belongs to the panel selected at the moment it began.
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

    if button == 1 && pressed && !crate::game::chetars_spatial_gui::is_2d_mode() {
        if crate::game::chetars_spatial_gui::can_drag_from_header() {
            if let Some(index) = crate::game::chetars_spatial_gui::hovered_panel() {
                crate::game::chetars_spatial_gui::set_selected_panel(index);
                crate::game::chetars_spatial_gui::load_panel(index);
                crate::game::chetars_spatial_gui::set_dragging(true);
                crate::game::chetars_spatial_gui::save_panel(index);
                *canceled = true;
            }
        }
        return;
    }

    if button != 0 || !pressed {
        return;
    }

    let Some(index) = crate::game::chetars_spatial_gui::hovered_panel() else {
        return;
    };
    crate::game::chetars_spatial_gui::set_selected_panel(index);

    // The plus button creates an independent panel with its own world position,
    // orientation, Follow/Keep mode and distance. The tenth panel is the hard cap.
    if crate::game::chetars_spatial_gui::hovered_add()
        || crate::game::chetars_spatial_gui::hovered_mode_action() == 7
    {
        let _ = crate::game::chetars_spatial_gui::add_panel();
        *canceled = true;
        return;
    }

    if let Some(tab) = crate::game::chetars_spatial_gui::hovered_navigation() {
        crate::game::chetars_spatial_gui::select_button(tab);
        *canceled = true;
        return;
    }

    crate::game::chetars_spatial_gui::load_panel(index);
    match crate::game::chetars_spatial_gui::hovered_mode_action() {
        1 => {
            crate::game::chetars_spatial_gui::toggle_dropdown();
            *canceled = true;
        }
        2 => {
            crate::game::chetars_spatial_gui::set_follow_mode(true);
            crate::game::chetars_spatial_gui::close_dropdown();
            *canceled = true;
        }
        3 => {
            crate::game::chetars_spatial_gui::set_follow_mode(false);
            crate::game::chetars_spatial_gui::close_dropdown();
            *canceled = true;
        }
        4 => {
            crate::game::chetars_spatial_gui::toggle_2d_mode();
            *canceled = true;
        }
        5 => {
            crate::game::chetars_spatial_gui::adjust_panel_distance(-0.25);
            *canceled = true;
        }
        6 => {
            crate::game::chetars_spatial_gui::adjust_panel_distance(0.25);
            *canceled = true;
        }
        _ => {}
    }
    crate::game::chetars_spatial_gui::save_panel(index);
}
