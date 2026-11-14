fn render(
    game: &crate::game::Game<'_>,
    batch: &mut HudBatch,
    gui_hidden: bool,
    focused: bool,
) {
    let _ = (gui_hidden, focused);

    if !game.native_playing() {
        return;
    }

    // No screen is opened: the actual mouse stays captured by gameplay.
    // The center of the view is the visual pointer/crosshair.
    crate::game::chetars_spatial_gui::set_viewport(
        batch.width,
        batch.height,
        [batch.width * 0.5, batch.height * 0.5],
    );
}
