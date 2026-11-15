fn mouse_button(
    game: &mut crate::game::Game<'_>,
    button: i32,
    pressed: bool,
    modifiers: u32,
    canceled: &mut bool,
) {
    let _ = modifiers;

    // Resize operations end on the matching mouse release even if the pointer
    // has already moved outside the panel.
    if button == 0 && !pressed && crate::game::chetars_spatial_gui::is_2d_resizing() {
        crate::game::chetars_spatial_gui::end_2d_resize();
        *canceled = true;
        return;
    }
    if button == 0 && !pressed
        && (0..crate::game::chetars_spatial_gui::max_panels())
            .any(|index| crate::game::chetars_spatial_gui::is_3d_resizing_for(index))
    {
        crate::game::chetars_spatial_gui::end_3d_resize();
        *canceled = true;
        return;
    }

    // A captured RMB drag belongs to the panel selected at the moment it began.
    if button == 1 && !pressed {
        let was_dragging = crate::game::chetars_spatial_gui::is_dragging();
        let over_2d = crate::game::chetars_spatial_gui::is_2d_mode()
            && crate::game::chetars_spatial_gui::two_d_pointer_over();
        crate::game::chetars_spatial_gui::set_dragging(false);
        if was_dragging || over_2d {
            *canceled = true;
        }
        return;
    }

    if !game.native_playing() || !crate::game::chetars_spatial_gui::is_open() {
        return;
    }

    // Reassert a free system pointer for active 2D-panel input only; when the
    // overlay is closed, ordinary gameplay keeps the normal captured mouse.
    if crate::game::chetars_spatial_gui::is_2d_mode() {
        game.capture(false);
    }

    let in_2d_panel = crate::game::chetars_spatial_gui::is_2d_mode()
        && crate::game::chetars_spatial_gui::two_d_pointer_over();
    if crate::game::chetars_spatial_gui::is_2d_mode() && !in_2d_panel {
        return;
    }
    if in_2d_panel && button != 0 {
        *canceled = true;
        return;
    }
    if in_2d_panel && button == 0 && !pressed {
        *canceled = true;
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

    // The dotted upper-left corner starts a resize gesture. In 3D, moving the
    // captured mouse changes view yaw/pitch; in 2D, the released system cursor
    // changes the window size directly.
    if crate::game::chetars_spatial_gui::is_2d_mode()
        && crate::game::chetars_spatial_gui::hovered_mode_action() == 11
    {
        let (_, _, pointer) = crate::game::chetars_spatial_gui::viewport();
        crate::game::chetars_spatial_gui::begin_2d_resize(pointer);
        *canceled = true;
        return;
    }

    if !crate::game::chetars_spatial_gui::is_2d_mode() {
        if let Some(index) = crate::game::chetars_spatial_gui::hovered_resize_panel() {
            crate::game::chetars_spatial_gui::set_selected_panel(index);
            crate::game::chetars_spatial_gui::load_panel(index);
            let (yaw, pitch, _) = game.camera_orientation().to_euler(glam::EulerRot::YXZ);
            crate::game::chetars_spatial_gui::begin_3d_resize(index, yaw, pitch);
            crate::game::chetars_spatial_gui::save_panel(index);
            *canceled = true;
            return;
        }
    }

    let Some(index) = crate::game::chetars_spatial_gui::hovered_panel() else {
        return;
    };
    crate::game::chetars_spatial_gui::set_selected_panel(index);

    // The close button removes the selected panel. Keep one panel alive so the
    // interface never becomes orphaned; the freed slot can be reused by +.
    if crate::game::chetars_spatial_gui::hovered_mode_action() == 10 {
        let _ = crate::game::chetars_spatial_gui::remove_panel(index);
        *canceled = true;
        return;
    }

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
            crate::game::chetars_spatial_gui::end_2d_resize();
            crate::game::chetars_spatial_gui::end_3d_resize();
            let now_2d = crate::game::chetars_spatial_gui::toggle_2d_mode();
            // A 2D window needs a visible system cursor; returning to 3D restores mouse-look.
            game.capture(!now_2d);
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
        8 => {
            let _ = crate::game::chetars_spatial_gui::step_selected_panel(false);
            *canceled = true;
        }
        9 => {
            let _ = crate::game::chetars_spatial_gui::step_selected_panel(true);
            *canceled = true;
        }
        _ => {}
    }
    crate::game::chetars_spatial_gui::save_panel(index);
    if in_2d_panel {
        // Clicks in the 2D window belong to the panel, never to the world behind it.
        *canceled = true;
    }
}
