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

    // The native pointer is released when 2D mode is enabled; keep the
    // independent GUI pointer coordinates in sync with real CursorMoved events.
    crate::game::chetars_spatial_gui::update_2d_resize(pointer);

    let atlas = &game.gui.atlas;
    let width = batch.width;
    let height = batch.height;
    let size_factor = crate::game::chetars_spatial_gui::two_d_scale();
    let panel_w = (620.0 * size_factor).clamp(320.0, (width - 32.0).max(320.0));
    let panel_h = (420.0 * size_factor).clamp(260.0, (height - 32.0).max(260.0));
    let x = (width - panel_w) * 0.5;
    let y = (height - panel_h) * 0.5;
    crate::game::chetars_spatial_gui::set_2d_pointer_over(
        pointer[0] >= x && pointer[0] <= x + panel_w
            && pointer[1] >= y && pointer[1] <= y + panel_h
    );
    let selected = crate::game::chetars_spatial_gui::selected_panel() as usize;
    crate::game::chetars_spatial_gui::set_hovered_panel(selected);
    crate::game::chetars_spatial_gui::set_hovered_add(false);
    crate::game::chetars_spatial_gui::set_hovered_navigation(None);
    crate::game::chetars_spatial_gui::set_hovered_mode_action(0);

    crate::game::chetars_spatial_gui::load_panel(selected);

    let fill = |batch: &mut HudBatch, x: f32, y: f32, w: f32, h: f32, color: u32| {
        batch.fill(atlas, x, y, w, h, color);
    };
    let text = |batch: &mut HudBatch, x: f32, y: f32, value: &str, color: u32| {
        batch.text(atlas, value, x, y, color, true);
    };

    // Full-screen 2D control window. The 3D panels continue rendering in the world behind it.
    fill(batch, 0.0, 0.0, width, height, 0x66000000);
    fill(batch, x + 7.0, y + 9.0, panel_w, panel_h, 0x99000000);
    fill(batch, x, y, panel_w, panel_h, 0xFF111820);
    fill(batch, x + 5.0, y + 5.0, panel_w - 10.0, panel_h - 10.0, 0xFF242C35);
    fill(batch, x + 12.0, y + 12.0, panel_w - 24.0, 48.0, 0xFF101820);
    fill(batch, x + 14.0, y + 70.0, 142.0, panel_h - 84.0, 0xFF1B242E);
    fill(batch, x + 166.0, y + 70.0, panel_w - 180.0, panel_h - 84.0, 0xFF3D4853);
    fill(batch, x + 172.0, y + 76.0, panel_w - 192.0, panel_h - 96.0, 0xFFF8FAFC);
    fill(batch, x, y, 3.0, panel_h, 0xFF33AAFF);
    fill(batch, x, y, panel_w, 3.0, 0xFF33AAFF);

    // Dotted triangular grip in the upper-left corner resizes the 2D window.
    let resize_handle_over = pointer[0] >= x + 3.0 && pointer[0] <= x + 34.0
        && pointer[1] >= y + 3.0 && pointer[1] <= y + 34.0;
    let resize_color = if resize_handle_over || crate::game::chetars_spatial_gui::is_2d_resizing() {
        0xFF33AAFF
    } else {
        0xFFB8C4D0
    };
    for row in 0..5 {
        for col in 0..=row {
            fill(
                batch,
                x + 9.0 + col as f32 * 4.2,
                y + 9.0 + row as f32 * 4.2,
                2.2,
                2.2,
                resize_color,
            );
        }
    }

    text(batch, x + 48.0, y + 28.0, "CHETARS", 0xFFFFFFFF);
    text(
        batch,
        x + 112.0,
        y + 28.0,
        &format!("PANEL {} / {}", selected + 1, crate::game::chetars_spatial_gui::panel_count()),
        0xFFB8C4D0,
    );

    // Compact previous/next controls select which independent 3D panel is
    // being edited while all 3D panels remain visible in the world behind this window.
    let panel_prev_x = x + 270.0;
    let panel_next_x = x + 304.0;
    let panel_arrow_y = y + 18.0;
    let over_prev = pointer[0] >= panel_prev_x && pointer[0] <= panel_prev_x + 26.0
        && pointer[1] >= panel_arrow_y && pointer[1] <= panel_arrow_y + 30.0;
    let over_next = pointer[0] >= panel_next_x && pointer[0] <= panel_next_x + 26.0
        && pointer[1] >= panel_arrow_y && pointer[1] <= panel_arrow_y + 30.0;
    fill(batch, panel_prev_x, panel_arrow_y, 26.0, 30.0,
        if over_prev { 0xFF33AAFF } else { 0xFF354352 });
    fill(batch, panel_next_x, panel_arrow_y, 26.0, 30.0,
        if over_next { 0xFF33AAFF } else { 0xFF354352 });
    text(batch, panel_prev_x + 8.0, panel_arrow_y + 21.0, "<", 0xFFFFFFFF);
    text(batch, panel_next_x + 8.0, panel_arrow_y + 21.0, ">", 0xFFFFFFFF);
    let mut header_action = if over_prev { 8 } else if over_next { 9 } else { 0 };

    let add_x = x + panel_w - 94.0;
    let add_y = y + 19.0;
    fill(
        batch,
        add_x,
        add_y,
        34.0,
        30.0,
        if crate::game::chetars_spatial_gui::panel_count()
            >= crate::game::chetars_spatial_gui::max_panels()
        { 0xFF272D34 }
        else if pointer[0] >= add_x && pointer[0] <= add_x + 34.0
            && pointer[1] >= add_y && pointer[1] <= add_y + 30.0
        { 0xFF33AAFF } else { 0xFF354352 },
    );
    text(batch, add_x + 12.0, add_y + 21.0, "+", 0xFFFFFFFF);
    let over_close = crate::game::chetars_spatial_gui::panel_count() > 1
        && pointer[0] >= add_x + 40.0 && pointer[0] <= add_x + 74.0
        && pointer[1] >= add_y && pointer[1] <= add_y + 30.0;
    fill(
        batch,
        add_x + 40.0,
        add_y,
        34.0,
        30.0,
        if over_close { 0xFFCC3344 } else { 0xFF354352 },
    );
    text(batch, add_x + 51.0, add_y + 21.0, "x", 0xFFFFFFFF);

    let over_add = crate::game::chetars_spatial_gui::panel_count()
        < crate::game::chetars_spatial_gui::max_panels()
        && pointer[0] >= add_x && pointer[0] <= add_x + 34.0
        && pointer[1] >= add_y && pointer[1] <= add_y + 30.0;
    crate::game::chetars_spatial_gui::set_hovered_add(over_add);
    if resize_handle_over {
        header_action = 11;
    } else if over_close {
        header_action = 10;
    } else if over_add {
        header_action = 7;
    }

    let tabs = ["HOME", "PROFILE", "SETTINGS"];
    let mut hovered_tab = None;
    for i in 0..3 {
        let bx = x + 24.0;
        let by = y + 88.0 + i as f32 * 48.0;
        let over = pointer[0] >= bx && pointer[0] <= bx + 122.0
            && pointer[1] >= by && pointer[1] <= by + 38.0;
        if over { hovered_tab = Some(i); }
        fill(
            batch,
            bx,
            by,
            122.0,
            38.0,
            if crate::game::chetars_spatial_gui::selected_button() == i {
                0xFF238BCB
            } else if over {
                0xFF34495A
            } else {
                0xFF2B343E
            },
        );
        text(
            batch,
            bx + 13.0,
            by + 24.0,
            tabs[i],
            if crate::game::chetars_spatial_gui::selected_button() == i || over {
                0xFFFFFFFF
            } else {
                0xFFCAD2DB
            },
        );
    }
    crate::game::chetars_spatial_gui::set_hovered_navigation(hovered_tab);

    let content_x = x + 192.0;
    let content_y = y + 94.0;
    let content_w = panel_w - 236.0;
    let mode = if crate::game::chetars_spatial_gui::follow_mode() { "FOLLOW" } else { "KEEP" };
    let distance = crate::game::chetars_spatial_gui::panel_distance();

    match crate::game::chetars_spatial_gui::selected_button() {
        0 => {
            text(batch, content_x, content_y, "FUNCTION CATALOG", 0xFF263442);
            for (i, name) in ["WORLD SPACE DISPLAY", "MULTI-PANEL WORKSPACE", "INDEPENDENT PLACEMENT", "2D / 3D PRESENTATION"].iter().enumerate() {
                let cy = content_y + 34.0 + i as f32 * 48.0;
                fill(batch, content_x, cy, content_w, 38.0, 0xFFEDF2F6);
                fill(batch, content_x, cy, 3.0, 38.0, 0xFF33AAFF);
                text(batch, content_x + 12.0, cy + 24.0, name, 0xFF202833);
            }
        }
        1 => {
            text(batch, content_x, content_y, "PANEL PROFILE", 0xFF263442);
            text(batch, content_x, content_y + 48.0, &format!("PLACEMENT: {}", mode), 0xFF202833);
            text(batch, content_x, content_y + 88.0, &format!("DISTANCE: {:.2} BLOCKS", distance), 0xFF202833);
            text(batch, content_x, content_y + 128.0, "RMB ON THE HEADER TO MOVE THE PANEL", 0xFF687482);
            text(batch, content_x, content_y + 158.0, "UP TO 10 PANELS CAN EXIST INDEPENDENTLY", 0xFF687482);
        }
        2 => {
            text(batch, content_x, content_y, "SETTINGS", 0xFF263442);
            let row_x = content_x;
            let row_w = content_w;
            let mode_y = content_y + 28.0;
            fill(batch, row_x, mode_y, row_w, 42.0, 0xFFE8EEF4);
            text(batch, row_x + 14.0, mode_y + 26.0, "FOLLOW / KEEP", 0xFF202833);
            text(batch, row_x + row_w - 104.0, mode_y + 26.0, mode, 0xFF167DAB);
            text(batch, row_x + row_w - 28.0, mode_y + 26.0,
                if crate::game::chetars_spatial_gui::dropdown_open() { "^" } else { "v" }, 0xFF303844);

            let mode_over = pointer[0] >= row_x && pointer[0] <= row_x + row_w
                && pointer[1] >= mode_y && pointer[1] <= mode_y + 42.0;
            let mut action = if mode_over { 1 } else { 0 };
            if crate::game::chetars_spatial_gui::dropdown_open() {
                for (i, (label, option)) in [("FOLLOW", 2u8), ("KEEP", 3u8)].into_iter().enumerate() {
                    let oy = mode_y + 48.0 + i as f32 * 36.0;
                    let over = pointer[0] >= row_x + 12.0 && pointer[0] <= row_x + row_w - 12.0
                        && pointer[1] >= oy && pointer[1] <= oy + 32.0;
                    fill(batch, row_x + 12.0, oy, row_w - 24.0, 32.0,
                        if over { 0xFFDCEBFA } else { 0xFFF2F4F7 });
                    text(batch, row_x + 24.0, oy + 21.0, label, 0xFF202833);
                    if over { action = option; }
                }
            }

            let two_y = content_y + 156.0;
            let two_d = crate::game::chetars_spatial_gui::is_2d_mode();
            fill(batch, row_x, two_y, row_w, 40.0, 0xFFE8EEF4);
            text(batch, row_x + 14.0, two_y + 25.0, "2D PANEL", 0xFF202833);
            text(batch, row_x + row_w - 52.0, two_y + 25.0, if two_d { "ON" } else { "OFF" },
                if two_d { 0xFF167D48 } else { 0xFF687482 });
            let two_over = pointer[0] >= row_x && pointer[0] <= row_x + row_w
                && pointer[1] >= two_y && pointer[1] <= two_y + 40.0;
            if two_over { action = 4; }

            let distance_y = content_y + 210.0;
            fill(batch, row_x, distance_y, row_w, 38.0, 0xFFE8EEF4);
            text(batch, row_x + 14.0, distance_y + 24.0, "DISTANCE", 0xFF202833);
            text(batch, row_x + row_w - 130.0, distance_y + 24.0, &format!("{:.1}", distance), 0xFF303844);
            let minus_x = row_x + row_w - 82.0;
            let plus_x = row_x + row_w - 38.0;
            fill(batch, minus_x, distance_y + 3.0, 34.0, 32.0, 0xFF4B5563);
            fill(batch, plus_x, distance_y + 3.0, 34.0, 32.0, 0xFF4B5563);
            text(batch, minus_x + 12.0, distance_y + 24.0, "-", 0xFFFFFFFF);
            text(batch, plus_x + 12.0, distance_y + 24.0, "+", 0xFFFFFFFF);
            if pointer[1] >= distance_y && pointer[1] <= distance_y + 38.0 {
                if pointer[0] >= minus_x && pointer[0] <= minus_x + 34.0 { action = 5; }
                if pointer[0] >= plus_x && pointer[0] <= plus_x + 34.0 { action = 6; }
            }
            crate::game::chetars_spatial_gui::set_hovered_mode_action(
                if header_action != 0 { header_action } else { action }
            );
        }
        _ => {
            crate::game::chetars_spatial_gui::set_hovered_mode_action(header_action);
        }
    }

    crate::game::chetars_spatial_gui::save_panel(selected);

    // A drawn cursor remains visible even on platforms where the native
    // pointer icon is suppressed by the compositor during a capture transition.
    let cx = pointer[0].clamp(0.0, width - 2.0);
    let cy = pointer[1].clamp(0.0, height - 2.0);
    for step in 0..11 {
        fill(batch, cx + step as f32, cy + step as f32, 2.0, 2.0, 0xFF101820);
        if step < 8 {
            fill(batch, cx + step as f32 + 1.0, cy + step as f32 + 1.0, 1.0, 1.0, 0xFFFFFFFF);
        }
    }
}
