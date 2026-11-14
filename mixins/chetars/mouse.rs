fn mouse(
    game: &crate::game::Game<'_>,
    screen: &NativeScreenView<'_>,
    button: i32,
    canceled: &mut bool,
) {
    if button != 0
        || screen.id != "mod"
        || game.native_mod_screen() != Some("chetars_spatial_gui")
    {
        return;
    }

    let width = screen.rect[2];
    let height = screen.rect[3];

    for i in 0..3 {
        if crate::game::chetars_spatial_gui::button_hit(screen.mouse, width, height, i) {
            crate::game::chetars_spatial_gui::select_button(
                crate::game::chetars_spatial_gui::button_at(screen.mouse, width, height),
            );
            *canceled = true;
            break;
        }
    }
}
