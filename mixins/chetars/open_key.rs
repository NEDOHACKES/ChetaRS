fn open_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    if pressed && key == "key.keyboard.f6" {
        let _ = game.native_mod_screen_open("chetars_spatial_gui");
    }
}
