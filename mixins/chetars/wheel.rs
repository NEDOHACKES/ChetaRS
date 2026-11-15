fn wheel(
    game: &mut crate::game::Game<'_>,
    steps: f32,
    claimed: &mut bool,
) {
    if !game.native_playing() || !crate::game::chetars_spatial_gui::is_open()
        || !steps.is_finite() || steps == 0.0
    {
        return;
    }

    let has_panel_target = crate::game::chetars_spatial_gui::hovered_panel().is_some();
    let over_zoom_area = if crate::game::chetars_spatial_gui::is_2d_mode() {
        crate::game::chetars_spatial_gui::two_d_pointer_over()
            && crate::game::chetars_spatial_gui::selected_button() == 2
    } else {
        has_panel_target
            && (crate::game::chetars_spatial_gui::can_drag_from_header()
                || crate::game::chetars_spatial_gui::selected_button() == 2)
    };

    if over_zoom_area {
        let index = crate::game::chetars_spatial_gui::hovered_panel()
            .unwrap_or(crate::game::chetars_spatial_gui::selected_panel() as usize);
        crate::game::chetars_spatial_gui::set_selected_panel(index);
        crate::game::chetars_spatial_gui::load_panel(index);
        crate::game::chetars_spatial_gui::adjust_panel_distance(-steps * 0.25);
        crate::game::chetars_spatial_gui::save_panel(index);
        *claimed = true;
    }
}
