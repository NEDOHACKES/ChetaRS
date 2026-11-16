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
    let scale = crate::game::chetars_spatial_gui::two_d_scale();

    let panel_w = ((width * 0.88).min(1160.0) * scale)
        .min((width - 28.0).max(420.0))
        .max(420.0);
    let panel_h = ((height * 0.84).min(720.0) * scale)
        .min((height - 28.0).max(300.0))
        .max(300.0);
    let x = (width - panel_w) * 0.5;
    let y = (height - panel_h) * 0.5;

    let fill = |batch: &mut HudBatch, x: f32, y: f32, w: f32, h: f32, color: u32| {
        batch.fill(atlas, x, y, w.max(0.0), h.max(0.0), color);
    };
    let text = |batch: &mut HudBatch, x: f32, y: f32, value: &str, color: u32| {
        batch.text(atlas, value, x, y, color, true);
    };

    // Glazed-inspired palette: deep navy glass, indigo surfaces and violet/blue accents.
    let bg = 0xFF121827;
    let frame = 0xFF1B243B;
    let card = 0xFF252F4A;
    let card2 = 0xFF2D3855;
    let hover = 0xFF3E4A70;
    let active = 0xFF4B4E8D;
    let purple = 0xFF805DFF;
    let blue = 0xFF6E9EFF;
    let white = 0xFFF3F5FF;
    let bright = 0xFFB8C2EA;
    let muted = 0xFF7F8BAF;
    let line = 0xFF39445F;
    let off = 0xFF4A536D;

    // Keep the Minecraft world visible and keep all independent 3D panels rendered behind this UI.
    fill(batch, 0.0, 0.0, width, height, 0xA90A0E18);

    // Outer glass frame and subtle edge highlights.
    fill(batch, x + 7.0, y + 9.0, panel_w, panel_h, 0x99000000);
    fill(batch, x, y, panel_w, panel_h, bg);
    fill(batch, x + 2.0, y + 2.0, panel_w - 4.0, 2.0, 0xFF303B62);
    fill(batch, x + 2.0, y + panel_h - 4.0, panel_w - 4.0, 2.0, 0xFF27314B);
    fill(batch, x + 2.0, y + 2.0, 2.0, panel_h - 4.0, purple);
    fill(batch, x + panel_w - 4.0, y + 2.0, 2.0, panel_h - 4.0, blue);

    // Header.
    let header_x = x + 16.0;
    let header_y = y + 16.0;
    let header_w = panel_w - 32.0;
    fill(batch, header_x, header_y, header_w, 56.0, frame);
    fill(batch, header_x, header_y, 2.0, 56.0, purple);
    fill(batch, header_x + 2.0, header_y, header_w - 4.0, 2.0, 0xFF344064);

    // Dotted top-left resize grip.
    let grip_over = pointer[0] >= x + 2.0 && pointer[0] <= x + 36.0
        && pointer[1] >= y + 2.0 && pointer[1] <= y + 36.0;
    let grip_color = if grip_over || crate::game::chetars_spatial_gui::is_2d_resizing() {
        blue
    } else {
        bright
    };
    for row in 0..5 {
        for col in 0..=row {
            fill(
                batch,
                x + 10.0 + col as f32 * 4.0,
                y + 10.0 + row as f32 * 4.0,
                2.2,
                2.2,
                grip_color,
            );
        }
    }

    // Brand.
    fill(batch, header_x + 18.0, header_y + 12.0, 32.0, 32.0, purple);
    fill(batch, header_x + 34.0, header_y + 12.0, 16.0, 32.0, blue);
    text(batch, header_x + 60.0, header_y + 23.0, "CHETARS", white);
    text(batch, header_x + 60.0, header_y + 40.0, "v0.5.5", muted);

    // Search field is visual; no new backend/search behavior is introduced.
    let search_x = header_x + header_w * 0.37;
    let search_w = header_w * 0.36;
    fill(batch, search_x, header_y + 10.0, search_w, 36.0, card);
    fill(batch, search_x, header_y + 10.0, 2.0, 36.0, 0xFF39456B);
    text(batch, search_x + 16.0, header_y + 33.0, "⌕", bright);
    text(batch, search_x + 42.0, header_y + 33.0, "Search modules...", muted);

    // Header controls.
    let pin_x = header_x + header_w - 92.0;
    let expand_x = pin_x + 34.0;
    let close_x = pin_x + 68.0;
    for (bx, glyph) in [(pin_x, "P"), (expand_x, "↗"), (close_x, "×")] {
        fill(batch, bx, header_y + 12.0, 28.0, 32.0, card2);
        text(batch, bx + 9.0, header_y + 33.0, glyph, bright);
    }

    let content_y = header_y + 68.0;
    let content_h = panel_h - 84.0;
    let gap = 12.0;
    let sidebar_w = (panel_w * 0.18).clamp(132.0, 210.0);
    let list_w = (panel_w * 0.27).clamp(205.0, 315.0);
    let settings_w = (panel_w * 0.34).clamp(255.0, 390.0);
    let info_w = panel_w - 32.0 - sidebar_w - list_w - settings_w - gap * 3.0;

    let sidebar_x = x + 16.0;
    let list_x = sidebar_x + sidebar_w + gap;
    let settings_x = list_x + list_w + gap;
    let info_x = settings_x + settings_w + gap;

    // Four-column glass layout matching the supplied reference.
    fill(batch, sidebar_x, content_y, sidebar_w, content_h, frame);
    fill(batch, list_x, content_y, list_w, content_h, frame);
    fill(batch, settings_x, content_y, settings_w, content_h, frame);
    fill(batch, info_x, content_y, info_w, content_h, frame);

    // Left navigation.
    let nav_items = [
        ("01", "Overview"),
        ("02", "Profile"),
        ("03", "Settings"),
    ];
    let selected_nav = crate::game::chetars_spatial_gui::selected_button().min(2);
    let mut hovered_nav = None;

    text(batch, sidebar_x + 18.0, content_y + 26.0, "MODULES", white);
    text(batch, sidebar_x + 18.0, content_y + 46.0, "Spatial control", muted);

    for i in 0..3 {
        let bx = sidebar_x + 12.0;
        let by = content_y + 64.0 + i as f32 * 50.0;
        let bw = sidebar_w - 24.0;
        let bh = 38.0;
        let over = pointer[0] >= bx && pointer[0] <= bx + bw
            && pointer[1] >= by && pointer[1] <= by + bh;
        if over {
            hovered_nav = Some(i);
        }
        let is_active = selected_nav == i;
        fill(
            batch,
            bx,
            by,
            bw,
            bh,
            if is_active { active } else if over { hover } else { card },
        );
        if is_active {
            fill(batch, bx, by, 3.0, bh, blue);
        }
        text(batch, bx + 12.0, by + 24.0, nav_items[i].0, if is_active { blue } else { muted });
        text(batch, bx + 43.0, by + 24.0, nav_items[i].1, if is_active || over { white } else { bright });
    }
    crate::game::chetars_spatial_gui::set_hovered_navigation(hovered_nav);

    let footer_y = content_y + content_h - 122.0;
    fill(batch, sidebar_x + 12.0, footer_y, sidebar_w - 24.0, 1.0, line);
    fill(batch, sidebar_x + 18.0, footer_y + 18.0, 36.0, 36.0, purple);
    text(batch, sidebar_x + 64.0, footer_y + 34.0, "ChetaRS", white);
    text(batch, sidebar_x + 64.0, footer_y + 50.0, "Spatial UI", muted);

    // Four small utility buttons are visual affordances; only existing menu functions are wired.
    let utility_y = footer_y + 70.0;
    for i in 0..4 {
        let bx = sidebar_x + 12.0 + i as f32 * ((sidebar_w - 24.0) / 4.0);
        let bw = (sidebar_w - 30.0) / 4.0;
        fill(batch, bx, utility_y, bw, 30.0, card2);
        text(batch, bx + bw * 0.5 - 4.0, utility_y + 20.0, ["⌕", "⚙", "◎", "→"][i], bright);
    }

    // Center-left: independent world panels, styled like the reference module list.
    let selected = crate::game::chetars_spatial_gui::selected_panel() as usize;
    text(batch, list_x + 20.0, content_y + 28.0, "SPATIAL PANELS", white);
    text(
        batch,
        list_x + 20.0,
        content_y + 48.0,
        &format!("{} active modules", crate::game::chetars_spatial_gui::panel_count()),
        muted,
    );

    let rows_top = content_y + 62.0;
    let rows_bottom = content_y + content_h - 18.0;
    let row_gap = 7.0;
    let max_rows = 7usize;
    let row_h = ((rows_bottom - rows_top - row_gap * (max_rows as f32 - 1.0)) / max_rows as f32)
        .clamp(42.0, 66.0);

    let mut visible_row = 0usize;
    let mut hovered_panel_action = 0u8;

    for target in 0..crate::game::chetars_spatial_gui::max_panels() {
        if !crate::game::chetars_spatial_gui::panel_exists(target) {
            continue;
        }
        if visible_row >= max_rows {
            break;
        }

        let ry = rows_top + visible_row as f32 * (row_h + row_gap);
        visible_row += 1;
        let rx = list_x + 12.0;
        let rw = list_w - 24.0;
        let over = pointer[0] >= rx && pointer[0] <= rx + rw
            && pointer[1] >= ry && pointer[1] <= ry + row_h;
        let is_active = selected == target;

        if over {
            hovered_panel_action = 12 + target as u8;
        }

        fill(
            batch,
            rx,
            ry,
            rw,
            row_h,
            if is_active { active } else if over { hover } else { card },
        );
        fill(batch, rx, ry, 3.0, row_h, if is_active { blue } else { purple });

        fill(batch, rx + 14.0, ry + 10.0, 34.0, 34.0, if is_active { purple } else { card2 });
        text(
            batch,
            rx + 25.0,
            ry + 32.0,
            if is_active { "◎" } else { "◌" },
            blue,
        );

        text(
            batch,
            rx + 60.0,
            ry + 25.0,
            &format!("Panel {:02}", target + 1),
            if is_active { white } else { bright },
        );
        text(
            batch,
            rx + 60.0,
            ry + 44.0,
            if crate::game::chetars_spatial_gui::panel_follow_mode(target) {
                "Follow"
            } else {
                "Keep at coordinates"
            },
            muted,
        );

        let mode_x = rx + rw - 78.0;
        fill(batch, mode_x, ry + 16.0, 58.0, 24.0, if crate::game::chetars_spatial_gui::panel_follow_mode(target) { blue } else { off });
        fill(
            batch,
            mode_x + if crate::game::chetars_spatial_gui::panel_follow_mode(target) { 31.0 } else { 3.0 },
            ry + 19.0,
            18.0,
            18.0,
            white,
        );
    }

    // Main settings column.
    let row_left = settings_x + 18.0;
    let row_w = settings_w - 36.0;
    let follow = crate::game::chetars_spatial_gui::follow_mode();
    let distance = crate::game::chetars_spatial_gui::panel_distance();
    let scale_label = format!("{:.0}%", crate::game::chetars_spatial_gui::panel_scale() * 100.0);

    fill(batch, settings_x + 10.0, content_y + 10.0, settings_w - 20.0, 56.0, card);
    fill(batch, settings_x + 24.0, content_y + 20.0, 36.0, 36.0, purple);
    text(batch, settings_x + 72.0, content_y + 34.0, &format!("Panel {:02}", selected + 1), white);
    text(
        batch,
        settings_x + 72.0,
        content_y + 51.0,
        if follow { "Follow mode" } else { "Keep mode" },
        muted,
    );

    let switch_x = settings_x + settings_w - 76.0;
    fill(batch, switch_x, content_y + 22.0, 48.0, 26.0, if follow { blue } else { off });
    fill(batch, switch_x + if follow { 25.0 } else { 3.0 }, content_y + 25.0, 20.0, 20.0, white);

    text(batch, row_left, content_y + 92.0, "GENERAL", blue);

    // Follow / Keep is the existing mode function, not a new module toggle.
    let mode_y = content_y + 108.0;
    let mode_over = pointer[0] >= row_left && pointer[0] <= row_left + row_w
        && pointer[1] >= mode_y && pointer[1] <= mode_y + 38.0;
    fill(batch, row_left, mode_y, row_w, 38.0, if mode_over { hover } else { card });
    text(batch, row_left + 14.0, mode_y + 24.0, "Placement mode", bright);
    text(batch, row_left + row_w - 122.0, mode_y + 24.0, if follow { "FOLLOW" } else { "KEEP" }, blue);
    text(batch, row_left + row_w - 20.0, mode_y + 24.0, "v", white);

    if crate::game::chetars_spatial_gui::dropdown_open() {
        let option_y = mode_y + 43.0;
        for (i, label) in ["FOLLOW", "KEEP AT COORDINATES"].iter().enumerate() {
            let oy = option_y + i as f32 * 34.0;
            let over = pointer[0] >= row_left + 6.0 && pointer[0] <= row_left + row_w - 6.0
                && pointer[1] >= oy && pointer[1] <= oy + 30.0;
            fill(batch, row_left + 6.0, oy, row_w - 12.0, 30.0, if over { hover } else { card2 });
            text(batch, row_left + 16.0, oy + 20.0, label, white);
        }
    }

    let mode_action = if mode_over {
        1
    } else if crate::game::chetars_spatial_gui::dropdown_open() {
        let option_y = mode_y + 43.0;
        if pointer[0] >= row_left + 6.0 && pointer[0] <= row_left + row_w - 6.0
            && pointer[1] >= option_y && pointer[1] <= option_y + 30.0 {
            2
        } else if pointer[0] >= row_left + 6.0 && pointer[0] <= row_left + row_w - 6.0
            && pointer[1] >= option_y + 34.0 && pointer[1] <= option_y + 64.0 {
            3
        } else {
            0
        }
    } else {
        0
    };

    // Distance control.
    let distance_y = content_y + 174.0;
    text(batch, row_left, distance_y, "DISTANCE", blue);
    fill(batch, row_left, distance_y + 10.0, row_w, 34.0, card);
    text(batch, row_left + 14.0, distance_y + 32.0, &format!("{:.2} blocks", distance), white);

    let minus_x = row_left + row_w - 94.0;
    let plus_x = row_left + row_w - 48.0;
    let minus_over = pointer[0] >= minus_x && pointer[0] <= minus_x + 38.0
        && pointer[1] >= distance_y + 11.0 && pointer[1] <= distance_y + 42.0;
    let plus_over = pointer[0] >= plus_x && pointer[0] <= plus_x + 38.0
        && pointer[1] >= distance_y + 11.0 && pointer[1] <= distance_y + 42.0;
    fill(batch, minus_x, distance_y + 11.0, 38.0, 31.0, if minus_over { hover } else { card2 });
    fill(batch, plus_x, distance_y + 11.0, 38.0, 31.0, if plus_over { hover } else { card2 });
    text(batch, minus_x + 14.0, distance_y + 32.0, "-", white);
    text(batch, plus_x + 14.0, distance_y + 32.0, "+", white);

    // 2D/3D switch.
    let overlay_y = content_y + 242.0;
    let two_d = crate::game::chetars_spatial_gui::is_2d_mode();
    text(batch, row_left, overlay_y, "DISPLAY", blue);
    fill(batch, row_left, overlay_y + 10.0, row_w, 38.0, card);
    text(batch, row_left + 14.0, overlay_y + 34.0, "2D overlay", bright);
    let sw_x = row_left + row_w - 58.0;
    fill(batch, sw_x, overlay_y + 16.0, 46.0, 26.0, if two_d { purple } else { off });
    fill(batch, sw_x + if two_d { 23.0 } else { 3.0 }, overlay_y + 19.0, 20.0, 20.0, white);

    // Scale.
    let scale_y = content_y + 306.0;
    fill(batch, row_left, scale_y, row_w, 38.0, card);
    text(batch, row_left + 14.0, scale_y + 24.0, "Panel size", bright);
    text(batch, row_left + row_w - 58.0, scale_y + 24.0, &scale_label, white);
    text(batch, row_left, scale_y + 58.0, "Use the dotted corner grip to resize", muted);

    // Far-right: Configs + Keybinds, matching the supplied reference.
    let info_left = info_x + 14.0;
    let info_w_inner = info_w - 28.0;

    text(batch, info_left, content_y + 28.0, "Configs", white);
    let config_names = ["default", "panel-safe", "night"];
    for i in 0..3 {
        let cy = content_y + 46.0 + i as f32 * 58.0;
        fill(batch, info_left, cy, info_w_inner, 48.0, card);
        text(batch, info_left + 14.0, cy + 21.0, config_names[i], white);
        text(batch, info_left + 14.0, cy + 38.0, ["active", "saved", "saved"][i], muted);
    }

    let key_y = content_y + 236.0;
    text(batch, info_left, key_y, "Keybinds", white);
    let keybinds = [
        ("F6", "Open / Close"),
        ("F7", "2D / 3D"),
        ("RMB", "Orbit"),
        ("LMB", "Resize"),
        ("WHEEL", "Distance"),
    ];
    for (i, (key, name)) in keybinds.iter().enumerate() {
        let ky = key_y + 28.0 + i as f32 * 32.0;
        text(batch, info_left, ky, name, bright);
        fill(batch, info_left + info_w_inner - 64.0, ky - 17.0, 60.0, 24.0, card2);
        text(batch, info_left + info_w_inner - 52.0, ky, key, white);
    }

    // Existing menu actions are mapped to the same runtime functions as before.
    let mut action = 0u8;
    let header_close_over = pointer[0] >= close_x && pointer[0] <= close_x + 28.0
        && pointer[1] >= header_y + 12.0 && pointer[1] <= header_y + 44.0;
    let header_expand_over = pointer[0] >= expand_x && pointer[0] <= expand_x + 28.0
        && pointer[1] >= header_y + 12.0 && pointer[1] <= header_y + 44.0;
    if header_close_over {
        action = 13;
    } else if header_expand_over {
        action = 14;
    } else if grip_over {
        action = 11;
    } else if minus_over {
        action = 5;
    } else if plus_over {
        action = 6;
    } else if mode_action != 0 {
        action = mode_action;
    } else if pointer[0] >= row_left && pointer[0] <= row_left + row_w
        && pointer[1] >= overlay_y + 10.0 && pointer[1] <= overlay_y + 48.0 {
        action = 4;
    } else if hovered_panel_action != 0 {
        action = hovered_panel_action;
    }

    crate::game::chetars_spatial_gui::set_hovered_mode_action(action);

    // Keep the custom pointer visible when Minecraft's capture state changes.
    let cx = pointer[0].clamp(0.0, width - 2.0);
    let cy = pointer[1].clamp(0.0, height - 2.0);
    for step in 0..11 {
        fill(batch, cx + step as f32, cy + step as f32, 2.0, 2.0, 0xFF101522);
        if step < 8 {
            fill(batch, cx + step as f32 + 1.0, cy + step as f32 + 1.0, 1.0, 1.0, white);
        }
    }

    crate::game::chetars_spatial_gui::set_2d_pointer_over(
        pointer[0] >= x && pointer[0] <= x + panel_w
            && pointer[1] >= y && pointer[1] <= y + panel_h,
    );
}
