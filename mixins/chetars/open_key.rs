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

    if key == "key.keyboard.escape" {
        crate::game::chetars_spatial_gui::close();
        return;
    }

    let tab = crate::game::chetars_spatial_gui::selected_button();
    match key {
        "key.keyboard.arrowleft" => {
            crate::game::chetars_spatial_gui::select_button((tab + 2) % 3);
        }
        "key.keyboard.arrowright" => {
            crate::game::chetars_spatial_gui::select_button((tab + 1) % 3);
        }
        "key.keyboard.arrowup" if tab == 2 => {
            crate::game::chetars_spatial_gui::select_setting(0);
        }
        "key.keyboard.arrowdown" if tab == 2 => {
            crate::game::chetars_spatial_gui::select_setting(1);
        }
        "key.keyboard.enter" | "key.keyboard.keypad.enter" if tab == 2 => {
            crate::game::chetars_spatial_gui::toggle_selected_setting();
        }
        "key.keyboard.minus" | "key.keyboard.keypad.subtract" if tab == 2 => {
            crate::game::chetars_spatial_gui::adjust_panel_distance(-0.25);
        }
        "key.keyboard.equal" | "key.keyboard.keypad.add" if tab == 2 => {
            crate::game::chetars_spatial_gui::adjust_panel_distance(0.25);
        }
        _ => {}
    }
}
