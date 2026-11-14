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

    // Arrow keys switch tabs without consuming the number keys used for
    // Minecraft's hotbar selection.
    let current = crate::game::chetars_spatial_gui::selected_button();
    let next = match key {
        "key.keyboard.arrowleft" => Some((current + 2) % 3),
        "key.keyboard.arrowright" => Some((current + 1) % 3),
        _ => None,
    };
    if let Some(index) = next {
        crate::game::chetars_spatial_gui::select_button(index);
    }
}
