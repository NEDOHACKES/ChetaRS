fn wheel(
    game: &mut crate::game::Game<'_>,
    steps: f32,
    claimed: &mut bool,
) {
    if !game.native_playing() || !crate::game::chetars_spatial_gui::is_open() {
        return;
    }

    // Zoom only while aiming at the title bar, so the hotbar wheel remains
    // untouched everywhere else in the world.
    if crate::game::chetars_spatial_gui::can_drag_from_header() && steps.is_finite() && steps != 0.0 {
        crate::game::chetars_spatial_gui::adjust_panel_distance(-steps * 0.25);
        *claimed = true;
    }
}
