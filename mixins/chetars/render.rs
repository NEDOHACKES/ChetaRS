fn render(
    game: &crate::game::Game<'_>,
    batch: &mut HudBatch,
    gui_hidden: bool,
    focused: bool,
) {
    let _ = gui_hidden;

    if !focused {
        crate::game::chetars_spatial_gui::set_dragging(false);
    }

    if !game.native_playing() {
        return;
    }

    // The overlay is non-modal. The game keeps its captured mouse for
    // camera-look; the center of the view is used as the world-ray pointer.
    crate::game::chetars_spatial_gui::set_viewport(
        batch.width,
        batch.height,
        [batch.width * 0.5, batch.height * 0.5],
    );
}
