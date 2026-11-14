fn open_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    if !pressed {
        return;
    }

    let is_open = game.native_mod_screen() == Some("chetars_spatial_gui");

    if is_open && (key == "key.keyboard.f6" || key == "key.keyboard.escape") {
        game.native_mod_screen_close("chetars_spatial_gui");
        return;
    }

    if !is_open && key == "key.keyboard.f6" {
        let _ = game.native_mod_screen_open("chetars_spatial_gui");
    }
}
