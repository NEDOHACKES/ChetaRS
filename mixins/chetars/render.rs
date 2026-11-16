fn render(
    game: &crate::game::Game<'_>,
    batch: &mut HudBatch,
    gui_hidden: bool,
    focused: bool,
) {
    let _ = gui_hidden;

    if !focused {
        crate::game::chetars_spatial_gui::set_dragging(false);
        crate::game::chetars_spatial_gui::end_3d_resize();
        crate::game::chetars_spatial_gui::end_2d_resize();
    }
    if !game.native_playing() || !crate::game::chetars_spatial_gui::is_open() {
        crate::game::chetars_spatial_gui::set_2d_pointer_over(false);
        crate::game::chetars_spatial_gui::end_2d_resize();
        return;
    }

    let pointer = [game.cursor.0, game.cursor.1];
    crate::game::chetars_spatial_gui::set_viewport(batch.width, batch.height, pointer);

    if !crate::game::chetars_spatial_gui::is_2d_mode() {
        crate::game::chetars_spatial_gui::set_2d_pointer_over(false);
        crate::game::chetars_spatial_gui::end_2d_resize();
        return;
    }

    crate::game::chetars_spatial_gui::update_2d_resize(pointer);

    let atlas = &game.gui.atlas;
    let width = batch.width;
    let height = batch.height;
    let size_factor = crate::game::chetars_spatial_gui::two_d_scale();

    // Responsive 2D counterpart of the world-space four-column ClickGUI.
    let panel_w = ((width * 0.88).min(1080.0) * size_factor)
        .min((width - 32.0).max(320.0))
        .max(320.0);
    let panel_h = ((height * 0.84).min(700.0) * size_factor)
        .min((height - 32.0).max(260.0))
        .max(260.0);
    let x = (width - panel_w) * 0.5;
    let y = (height - panel_h) * 0.5;

    let section_x = x + 14.0;
    let section_y = y + 72.0;
    let section_h = panel_h - 86.0;
    let gap = 10.0;
    let usable_w = panel_w - 28.0 - gap * 3.0;
    let sidebar_w = usable_w * 0.16;
    let list_w = usable_w * 0.25;
    let settings_w = usable_w * 0.39;
    let info_w = usable_w - sidebar_w - list_w - settings_w;

    let sidebar_x = section_x;
    let list_x = sidebar_x + sidebar_w + gap;
    let settings_x = list_x + list_w + gap;
    let info_x = settings_x + settings_w + gap;
    let selected = crate::game::chetars_spatial_gui::selected_panel() as usize;

    let inside = pointer[0] >= x && pointer[0] <= x + panel_w
        && pointer[1] >= y && pointer[1] <= y + panel_h;
    crate::game::chetars_spatial_gui::set_2d_pointer_over(inside);
    crate::game::chetars_spatial_gui::set_hovered_panel(selected);
    crate::game::chetars_spatial_gui::set_hovered_add(false);
    crate::game::chetars_spatial_gui::set_hovered_navigation(None);
    crate::game::chetars_spatial_gui::set_hovered_mode_action(0);

    let fill = |batch: &mut HudBatch, x: f32, y: f32, w: f32, h: f32, color: u32| {
        batch.fill(atlas, x, y, w.max(0.0), h.max(0.0), color);
    };
    let text = |batch: &mut HudBatch, x: f32, y: f32, value: &str, color: u32| {
        batch.text(atlas, value, x, y, color, true);
    };
    let rect = |batch: &mut HudBatch, x: f32, y: f32, w: f32, h: f32, color: u32| {
        fill(batch, x, y, w, h, color);
    };

    let bg = 0xFF141A2A;
    let card = 0xFF242D43;
    let card_alt = 0xFF2C3650;
    let card_hover = 0xFF3C4567;
    let purple = 0xFF7C5CFF;
    let blue = 0xFF6EA8FF;
    let bright = 0xFFB8C2FF;
    let muted = 0xFF8794BA;
    let white = 0xFFF2F4FF;

    // Dim the Minecraft world without hiding the independent 3D panels behind this overlay.
    fill(batch, 0.0, 0.0, width, height, 0xA60A0F1F);

    // Window frame, top bar and four distinct work areas.
    fill(batch, x + 8.0, y + 10.0, panel_w, panel_h, 0x99000000);
    fill(batch, x, y, panel_w, panel_h, bg);
    fill(batch, x + 5.0, y + 5.0, panel_w - 10.0, panel_h - 10.0, 0xFF1B2336);
    fill(batch, x + 12.0, y + 12.0, panel_w - 24.0, 48.0, 0xFF202940);
    fill(batch, sidebar_x, section_y, sidebar_w, section_h, 0xFF202A40);
    fill(batch, list_x, section_y, list_w, section_h, 0xFF202A40);
    fill(batch, settings_x, section_y, settings_w, section_h, 0xFF242D43);
    fill(batch, info_x, section_y, info_w, section_h, 0xFF202A40);
    fill(batch, x, y, 3.0, panel_h, purple);
    fill(batch, x, y, panel_w, 3.0, blue);
    fill(batch, x + 5.0, y + panel_h - 5.0, panel_w - 10.0, 1.0, 0xFF46516C);

    // Dotted triangular grip stays in the top-left corner and controls window scale.
    let resize_handle_over = pointer[0] >= x + 3.0 && pointer[0] <= x + 34.0
        && pointer[1] >= y + 3.0 && pointer[1] <= y + 34.0;
    let resize_color = if resize_handle_over || crate::game::chetars_spatial_gui::is_2d_resizing() {
        blue
    } else {
        bright
    };
    for row in 0..5 {
        for col in 0..=row {
            fill(batch, x + 9.0 + col as f32 * 4.2, y + 9.0 + row as f32 * 4.2,
                2.2, 2.2, resize_color);
        }
    }

    // Header brand and panel navigation.
    fill(batch, x + 18.0, y + 15.0, 28.0, 28.0, purple);
    fill(batch, x + 33.0, y + 15.0, 13.0, 28.0, blue);
    text(batch, x + 54.0, y + 29.0, "CHETARS", white);
    text(batch, x + 116.0, y + 29.0, "SPATIAL CONTROL CENTER", muted);
    text(batch, x + 218.0, y + 29.0,
        &format!("PANEL {} / {}", selected + 1, crate::game::chetars_spatial_gui::panel_count()), bright);

    let arrow_y = y + 21.0;
    let prev_x = x + 12.0 + 244.0;
    let next_x = x + 12.0 + 280.0;
    let over_prev = pointer[0] >= prev_x && pointer[0] <= prev_x + 28.0
        && pointer[1] >= arrow_y && pointer[1] <= arrow_y + 30.0;
    let over_next = pointer[0] >= next_x && pointer[0] <= next_x + 28.0
        && pointer[1] >= arrow_y && pointer[1] <= arrow_y + 30.0;
    for (bx, glyph, over) in [(prev_x, "<", over_prev), (next_x, ">", over_next)] {
        fill(batch, bx, arrow_y, 28.0, 30.0, if over { card_hover } else { card_alt });
        text(batch, bx + 9.0, arrow_y + 21.0, glyph, white);
    }

    let add_x = x + panel_w - 124.0;
    let add_y = arrow_y;
    let can_add = crate::game::chetars_spatial_gui::panel_count()
        < crate::game::chetars_spatial_gui::max_panels();
    let over_add = can_add && pointer[0] >= add_x && pointer[0] <= add_x + 34.0
        && pointer[1] >= add_y && pointer[1] <= add_y + 30.0;
    let over_close = crate::game::chetars_spatial_gui::panel_count() > 1
        && pointer[0] >= add_x + 40.0 && pointer[0] <= add_x + 74.0
        && pointer[1] >= add_y && pointer[1] <= add_y + 30.0;
    fill(batch, add_x, add_y, 34.0, 30.0,
        if !can_add { 0xFF30394D } else if over_add { purple } else { card_alt });
    text(batch, add_x + 12.0, add_y + 21.0, "+", white);
    fill(batch, add_x + 40.0, add_y, 34.0, 30.0,
        if over_close { 0xFFB84B66 } else { card_alt });
    text(batch, add_x + 52.0, add_y + 21.0, "x", white);
    crate::game::chetars_spatial_gui::set_hovered_add(over_add);

    let mut header_action = if resize_handle_over { 11 }
        else if over_close { 10 }
        else if over_add { 7 }
        else if over_prev { 8 }
        else if over_next { 9 }
        else { 0 };

    // Left navigation rail.
    let tabs = ["OVERVIEW", "PROFILE", "SETTINGS"];
    let mut hovered_tab = None;
    for i in 0..3 {
        let bx = sidebar_x + 8.0;
        let by = section_y + 18.0 + i as f32 * 48.0;
        let bw = sidebar_w - 16.0;
        let bh = 38.0;
        let over = pointer[0] >= bx && pointer[0] <= bx + bw
            && pointer[1] >= by && pointer[1] <= by + bh;
        if over { hovered_tab = Some(i); }
        let active = crate::game::chetars_spatial_gui::selected_button() == i;
        fill(batch, bx, by, bw, bh,
            if active { 0xFF514C86 } else if over { card_hover } else { card });
        if active { fill(batch, bx, by, 3.0, bh, blue); }
        text(batch, bx + 12.0, by + 24.0, ["01", "02", "03"][i],
            if active { blue } else { muted });
        text(batch, bx + 40.0, by + 24.0, tabs[i],
            if active || over { white } else { bright });
    }
    crate::game::chetars_spatial_gui::set_hovered_navigation(hovered_tab);
    text(batch, sidebar_x + 12.0, section_y + section_h - 18.0, "WORLD UI", muted);

    // Center-left list of all active world panels.
    text(batch, list_x + 12.0, section_y + 23.0, "SPATIAL PANELS", white);
    text(batch, list_x + 12.0, section_y + 40.0, "Select one to edit", muted);
    let rows_top = section_y + 58.0;
    let rows_bottom = section_y + section_h - 10.0;
    let row_gap = 4.0;
    let row_h = ((rows_bottom - rows_top - row_gap * 9.0) / 10.0).clamp(22.0, 34.0);
    let mut visible_row = 0usize;
    let mut hovered_panel_action = 0u8;
    for target in 0..crate::game::chetars_spatial_gui::max_panels() {
        if !crate::game::chetars_spatial_gui::panel_exists(target) { continue; }
        let ry = rows_top + visible_row as f32 * (row_h + row_gap);
        visible_row += 1;
        let rx = list_x + 8.0;
        let rw = list_w - 16.0;
        let over = pointer[0] >= rx && pointer[0] <= rx + rw
            && pointer[1] >= ry && pointer[1] <= ry + row_h;
        let active = selected == target;
        if over { hovered_panel_action = 12 + target as u8; }
        fill(batch, rx, ry, rw, row_h,
            if active { 0xFF454A78 } else if over { card_hover } else { card_alt });
        fill(batch, rx, ry, 3.0, row_h, if active { blue } else { purple });
        text(batch, rx + 10.0, ry + row_h * 0.62, &format!("PANEL {:02}", target + 1),
            if active { white } else { bright });
        text(batch, rx + rw - 58.0, ry + row_h * 0.62,
            if crate::game::chetars_spatial_gui::panel_follow_mode(target) { "FOLLOW" } else { "KEEP" }, muted);
    }

    // Main settings/profile area.
    let row_left = settings_x + 12.0;
    let row_w = settings_w - 24.0;
    let follow = crate::game::chetars_spatial_gui::follow_mode();
    let mode_label = if follow { "FOLLOW" } else { "KEEP" };
    let distance = crate::game::chetars_spatial_gui::panel_distance();
    let scale_label = format!("{:.0}%", crate::game::chetars_spatial_gui::panel_scale() * 100.0);
    let mut settings_action = 0u8;

    match crate::game::chetars_spatial_gui::selected_button() {
        0 => {
            text(batch, settings_x + 14.0, section_y + 24.0, "OVERVIEW", white);
            text(batch, settings_x + 14.0, section_y + 42.0, "Your spatial workspace", muted);
            let cards = [
                (section_y + 64.0, "3D WORLD PANELS", "Floating displays in your world"),
                (section_y + 124.0, "360 ORBIT CONTROL", "Hold RMB on a panel header"),
                (section_y + 184.0, "INDEPENDENT PANELS", "Each panel keeps its own position"),
            ];
            for (cy, title, detail) in cards {
                fill(batch, row_left, cy, row_w, 48.0, card_alt);
                fill(batch, row_left, cy, 3.0, 48.0, purple);
                text(batch, row_left + 12.0, cy + 18.0, title, white);
                text(batch, row_left + 12.0, cy + 36.0, detail, muted);
            }
            text(batch, row_left, section_y + 270.0,
                &format!("ACTIVE PANELS: {} / {}", crate::game::chetars_spatial_gui::panel_count(), crate::game::chetars_spatial_gui::max_panels()), bright);
        }
        1 => {
            text(batch, settings_x + 14.0, section_y + 24.0, "PANEL PROFILE", white);
            text(batch, settings_x + 14.0, section_y + 42.0, "Selected display details", muted);
            let distance_profile = format!("{:.1} BLOCKS", distance);
            let cards = [
                (section_y + 68.0, "PLACEMENT MODE", mode_label),
                (section_y + 132.0, "PANEL DISTANCE", distance_profile.as_str()),
                (section_y + 196.0, "PANEL SCALE", scale_label.as_str()),
            ];
            for (cy, title, value) in cards {
                fill(batch, row_left, cy, row_w, 48.0, card_alt);
                text(batch, row_left + 12.0, cy + 18.0, title, muted);
                text(batch, row_left + 12.0, cy + 37.0, value, white);
            }
            text(batch, row_left, section_y + 280.0, "RMB ORBIT KEEPS THE PANEL UPRIGHT", bright);
        }
        _ => {
            text(batch, settings_x + 14.0, section_y + 24.0, "DISPLAY SETTINGS", white);
            text(batch, settings_x + 14.0, section_y + 42.0, "Configure the selected panel", muted);

            let mode_y = section_y + 50.0;
            let mode_open = crate::game::chetars_spatial_gui::dropdown_open();
            let mode_over = pointer[0] >= row_left && pointer[0] <= row_left + row_w
                && pointer[1] >= mode_y && pointer[1] <= mode_y + 36.0;
            fill(batch, row_left, mode_y, row_w, 36.0, if mode_open || mode_over { card_hover } else { card_alt });
            text(batch, row_left + 12.0, mode_y + 23.0, "FOLLOW / KEEP", bright);
            text(batch, row_left + row_w - 88.0, mode_y + 23.0, mode_label, blue);
            text(batch, row_left + row_w - 18.0, mode_y + 23.0,
                if mode_open { "^" } else { "v" }, white);
            if mode_over { settings_action = 1; }

            if mode_open {
                let option_y = section_y + 94.0;
                for (i, label_text) in ["FOLLOW", "KEEP AT COORDINATES"].iter().enumerate() {
                    let oy = option_y + i as f32 * 34.0;
                    let over = pointer[0] >= row_left + 8.0 && pointer[0] <= row_left + row_w - 8.0
                        && pointer[1] >= oy && pointer[1] <= oy + 30.0;
                    fill(batch, row_left + 8.0, oy, row_w - 16.0, 30.0,
                        if over { card_hover } else if (i == 0) == follow { 0xFF514C86 } else { card });
                    text(batch, row_left + 20.0, oy + 20.0, label_text, white);
                    if over { settings_action = if i == 0 { 2 } else { 3 }; }
                }
            }

            let two_d_y = section_y + 176.0;
            let two_d = crate::game::chetars_spatial_gui::is_2d_mode();
            let two_over = pointer[0] >= row_left && pointer[0] <= row_left + row_w
                && pointer[1] >= two_d_y && pointer[1] <= two_d_y + 36.0;
            fill(batch, row_left, two_d_y, row_w, 36.0, if two_over { card_hover } else { card_alt });
            text(batch, row_left + 12.0, two_d_y + 23.0, "2D OVERLAY", bright);
            let sw_x = row_left + row_w - 58.0;
            fill(batch, sw_x, two_d_y + 5.0, 46.0, 26.0, if two_d { purple } else { 0xFF4A5367 });
            fill(batch, sw_x + if two_d { 23.0 } else { 3.0 }, two_d_y + 8.0, 20.0, 20.0, white);
            if two_over { settings_action = 4; }

            let distance_y = section_y + 224.0;
            fill(batch, row_left, distance_y, row_w, 38.0, card_alt);
            text(batch, row_left + 12.0, distance_y + 24.0, "DISTANCE", bright);
            text(batch, row_left + row_w - 148.0, distance_y + 24.0, &format!("{:.1}", distance), white);
            let minus_x = settings_x + settings_w - 100.0;
            let plus_x = settings_x + settings_w - 54.0;
            let minus_over = pointer[0] >= minus_x && pointer[0] <= minus_x + 38.0
                && pointer[1] >= distance_y + 3.0 && pointer[1] <= distance_y + 35.0;
            let plus_over = pointer[0] >= plus_x && pointer[0] <= plus_x + 38.0
                && pointer[1] >= distance_y + 3.0 && pointer[1] <= distance_y + 35.0;
            fill(batch, minus_x, distance_y + 3.0, 38.0, 32.0, if minus_over { card_hover } else { card });
            fill(batch, plus_x, distance_y + 3.0, 38.0, 32.0, if plus_over { card_hover } else { card });
            text(batch, minus_x + 15.0, distance_y + 24.0, "-", white);
            text(batch, plus_x + 15.0, distance_y + 24.0, "+", white);
            if minus_over { settings_action = 5; }
            if plus_over { settings_action = 6; }
            text(batch, row_left, section_y + 288.0, "RESIZE WITH THE DOTTED CORNER GRIP", muted);
        }
    }

    // Compact status and real keyboard/mouse shortcuts.
    let info_y = section_y;
    let info_card_x = info_x + 8.0;
    let info_card_w = info_w - 16.0;
    fill(batch, info_card_x, info_y + 10.0, info_card_w, 150.0, card);
    text(batch, info_x + 18.0, info_y + 32.0, "PANEL STATUS", white);
    text(batch, info_x + 18.0, info_y + 60.0,
        &format!("{:02} / 10 ACTIVE", selected + 1), blue);
    text(batch, info_x + 18.0, info_y + 86.0,
        if follow { "FOLLOW MODE" } else { "KEEP MODE" }, bright);
    text(batch, info_x + 18.0, info_y + 112.0, &format!("{:.1} BLOCKS", distance), muted);
    text(batch, info_x + 18.0, info_y + 136.0, &format!("SCALE {}", scale_label), muted);

    let quick_y = info_y + 172.0;
    let quick_h = (section_y + section_h - quick_y - 10.0).max(104.0);
    fill(batch, info_card_x, quick_y, info_card_w, quick_h, card);
    text(batch, info_x + 18.0, quick_y + 22.0, "QUICK CONTROLS", white);
    for (i, (key, name)) in [
        ("F6", "OPEN / CLOSE"),
        ("F7", "2D / 3D"),
        ("RMB", "ORBIT"),
        ("LMB", "RESIZE"),
        ("WHEEL", "DISTANCE"),
    ].iter().enumerate() {
        let ky = quick_y + 48.0 + i as f32 * 32.0;
        text(batch, info_x + 18.0, ky, key, blue);
        text(batch, info_x + 66.0, ky, name, bright);
    }

    // Publish the current hover action after all hit regions have been tested.
    let action = if header_action != 0 { header_action }
        else if hovered_panel_action != 0 { hovered_panel_action }
        else { settings_action };
    crate::game::chetars_spatial_gui::set_hovered_mode_action(action);

    // A custom pointer is painted last so it remains visible on capture transitions.
    let cx = pointer[0].clamp(0.0, width - 2.0);
    let cy = pointer[1].clamp(0.0, height - 2.0);
    for step in 0..11 {
        fill(batch, cx + step as f32, cy + step as f32, 2.0, 2.0, 0xFF101522);
        if step < 8 {
            fill(batch, cx + step as f32 + 1.0, cy + step as f32 + 1.0, 1.0, 1.0, white);
        }
    }
}
