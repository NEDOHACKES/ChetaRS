fn open_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    if !pressed || key != "key.keyboard.f6" {
        return;
    }

    if game.native_mod_screen() == Some("chetars_spatial_gui") {
        game.native_mod_screen_close("chetars_spatial_gui");
    } else {
        let _ = game.native_mod_screen_open("chetars_spatial_gui");
    }
}
