fn open_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    if !pressed || !game.native_playing() {
        return;
    }

    if key == "key.keyboard.f6" {
        let now_open = crate::game::chetars_spatial_gui::toggle_open();
        if now_open && crate::game::chetars_spatial_gui::is_2d_mode() {
            game.capture(false);
        } else if !now_open {
            game.capture(true);
        }
        return;
    }

    if !crate::game::chetars_spatial_gui::is_open() {
        return;
    }

    if key == "key.keyboard.f7" {
        crate::game::chetars_spatial_gui::end_2d_resize();
        crate::game::chetars_spatial_gui::end_3d_resize();
        let now_2d = crate::game::chetars_spatial_gui::toggle_2d_mode();
        game.capture(!now_2d);
        return;
    }

    if key == "key.keyboard.escape" {
        crate::game::chetars_spatial_gui::end_2d_resize();
        crate::game::chetars_spatial_gui::end_3d_resize();
        if crate::game::chetars_spatial_gui::dropdown_open() {
            crate::game::chetars_spatial_gui::close_dropdown();
        } else {
            crate::game::chetars_spatial_gui::close();
            game.capture(true);
        }
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
            let index = crate::game::chetars_spatial_gui::selected_panel() as usize;
            crate::game::chetars_spatial_gui::load_panel(index);
            if crate::game::chetars_spatial_gui::dropdown_open() {
                crate::game::chetars_spatial_gui::toggle_selected_setting();
                crate::game::chetars_spatial_gui::close_dropdown();
                crate::game::chetars_spatial_gui::save_panel(index);
            } else {
                crate::game::chetars_spatial_gui::toggle_dropdown();
            }
        }
        "key.keyboard.minus" | "key.keyboard.keypad.subtract" if tab == 2 => {
            let index = crate::game::chetars_spatial_gui::selected_panel() as usize;
            crate::game::chetars_spatial_gui::load_panel(index);
            crate::game::chetars_spatial_gui::adjust_panel_distance(-0.25);
            crate::game::chetars_spatial_gui::save_panel(index);
        }
        "key.keyboard.equal" | "key.keyboard.keypad.add" if tab == 2 => {
            let index = crate::game::chetars_spatial_gui::selected_panel() as usize;
            crate::game::chetars_spatial_gui::load_panel(index);
            crate::game::chetars_spatial_gui::adjust_panel_distance(0.25);
            crate::game::chetars_spatial_gui::save_panel(index);
        }
        _ => {}
    }
}
