fn mouse(
    game: &crate::game::Game<'_>,
    screen: &NativeScreenView<'_>,
    button: i32,
    canceled: &mut bool,
) {
    if screen.id != "mod"
        || game.native_mod_screen() != Some("chetars_spatial_gui")
    {
        return;
    }

    let width = screen.rect[2];
    let height = screen.rect[3];

    // The white canvas is intentionally inert. Consume clicks anywhere on
    // the spatial panel so Minecraft never routes them to inventory slots.
    if !crate::game::chetars_spatial_gui::panel_hit(screen.mouse, width, height) {
        return;
    }

    if button == 0 {
        if let Some(index) =
            crate::game::chetars_spatial_gui::button_at(screen.mouse, width, height)
        {
            crate::game::chetars_spatial_gui::select_button(index);
        }
    }

    *canceled = true;
}
