fn open_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    if !pressed || !game.native_playing() {
        return;
    }

    if key == "key.keyboard.f6" {
        crate::game::chetars_spatial_gui::toggle_open();
        return;
    }

    if !crate::game::chetars_spatial_gui::is_open() {
        return;
    }

    // Escape hides the floating display. Vanilla may still open its pause
    // screen on this key press, since the display is a non-modal overlay.
    if key == "key.keyboard.escape" {
        crate::game::chetars_spatial_gui::close();
        return;
    }

    // Number keys switch the navigation rail while movement and mouse-look
    // remain fully available.
    let selection = match key {
        "key.keyboard.1" => Some(0),
        "key.keyboard.2" => Some(1),
        "key.keyboard.3" => Some(2),
        _ => None,
    };
    if let Some(index) = selection {
        crate::game::chetars_spatial_gui::select_button(index);
    }
}
