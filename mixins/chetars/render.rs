fn render(
    game: &crate::game::Game<'_>,
    batch: &mut HudBatch,
    screen: &NativeScreenView<'_>,
) {
    if screen.id != "mod"
        || game.native_mod_screen() != Some("chetars_spatial_gui")
    {
        return;
    }

    // The HUD hook only records the logical viewport and pointer. The menu
    // itself is submitted by world_submit.rs into Minecraft's 3D world pass.
    crate::game::chetars_spatial_gui::set_viewport(
        batch.width,
        batch.height,
        screen.mouse,
    );
}
