fn world_submit(
    game: &crate::game::Game<'_>,
    draw: &mut NativeWorldDraw,
) {
    // Keep one private entry-point per Horizon Mixins helper. The panel
    // renderer is a local closure so it remains part of this injection.
    let draw_panel = |game: &crate::game::Game<'_>,
                      draw: &mut NativeWorldDraw,
                      panel_index: usize| {
    if !game.native_playing() || !crate::game::chetars_spatial_gui::is_open()
        || !crate::game::chetars_spatial_gui::panel_exists(panel_index)
    {
        return;
    }

    let (width, height, mouse) = crate::game::chetars_spatial_gui::viewport();
    if width <= 0.0 || height <= 0.0 { return; }

    let panel = crate::game::chetars_spatial_gui::panel(mouse, width, height);
    let centre_x = width * 0.5;
    let centre_y = height * 0.5;
    let current_rotation = game.camera_orientation();
    let (camera_yaw, camera_pitch, _) = current_rotation.to_euler(glam::EulerRot::YXZ);
    let camera_position = glam::DVec3::from_array(draw.camera);

    if crate::game::chetars_spatial_gui::is_3d_resizing_for(panel_index) {
        crate::game::chetars_spatial_gui::update_3d_resize(
            panel_index,
            camera_yaw,
            camera_pitch,
        );
    } else if crate::game::chetars_spatial_gui::is_dragging() {
        crate::game::chetars_spatial_gui::update_drag_view(
            camera_yaw,
            camera_pitch,
            camera_position.to_array(),
        );
    }

    let follow = crate::game::chetars_spatial_gui::follow_mode();

    if !crate::game::chetars_spatial_gui::anchor_ready() {
        let forward = (current_rotation * glam::Vec3::NEG_Z).normalize();
        let base_yaw = forward.x.atan2(-forward.z);
        let spread = if panel_index == 0 {
            0.0
        } else if panel_index % 2 == 1 {
            ((panel_index + 1) / 2) as f32 * 1.10
        } else {
            -((panel_index / 2) as f32) * 1.10
        };
        let spawn_yaw = base_yaw + spread;
        let spawn_pitch = forward.y.clamp(-1.0, 1.0).asin();
        let spawn_direction = crate::game::chetars_spatial_gui::orbit_offset(
            spawn_yaw,
            spawn_pitch,
            1.0,
        ).normalize();
        let anchor_position = camera_position + spawn_direction.as_dvec3()
            * crate::game::chetars_spatial_gui::panel_distance() as f64;
        let toward_player = -spawn_direction;
        // Keep the UI upright: orbit elevation changes panel position, never its tilt.
        let spawn_rotation = glam::Quat::from_rotation_y(
            toward_player.x.atan2(toward_player.z),
        );
        crate::game::chetars_spatial_gui::save_anchor(
            anchor_position.to_array(),
            spawn_rotation,
        );
        crate::game::chetars_spatial_gui::set_orbit_angles(spawn_yaw, spawn_pitch);
    }

    // Switching modes must not snap the panel. Capture its current world
    // point, then reinterpret that same point as an offset from the player
    // (Follow) or store the current follow offset as a world anchor (Pinned).
    if crate::game::chetars_spatial_gui::take_follow_mode_changed() {
        let (saved_position, saved_rotation) = crate::game::chetars_spatial_gui::anchor();
        let relative = glam::DVec3::from_array(saved_position) - camera_position;
        let radius = relative.length();

        if follow {
            if radius.is_finite() && radius > 0.05 {
                crate::game::chetars_spatial_gui::set_panel_distance(radius as f32);
                let direction = relative / radius;
                crate::game::chetars_spatial_gui::set_orbit_angles(
                    direction.x.atan2(-direction.z) as f32,
                    direction.y.clamp(-1.0, 1.0).asin() as f32,
                );
            }
        } else {
            let (orbit_yaw, orbit_pitch) = crate::game::chetars_spatial_gui::orbit_angles();
            let offset = crate::game::chetars_spatial_gui::orbit_offset(
                orbit_yaw,
                orbit_pitch,
                crate::game::chetars_spatial_gui::panel_distance(),
            );
            let new_position = camera_position + offset.as_dvec3();
            let toward_player = (camera_position - new_position).as_vec3().normalize_or_zero();
            let facing = glam::Quat::from_rotation_y(toward_player.x.atan2(toward_player.z));
            crate::game::chetars_spatial_gui::save_anchor(
                new_position.to_array(),
                facing,
            );
        }
    }

    let distance_dirty = crate::game::chetars_spatial_gui::take_distance_dirty();
    let distance = crate::game::chetars_spatial_gui::panel_distance();

    // A distance adjustment moves a pinned panel along its current radial
    // direction. Follow mode derives the same point from the saved player offset.
    if distance_dirty
        && !follow
        && !crate::game::chetars_spatial_gui::is_dragging()
        && crate::game::chetars_spatial_gui::anchor_ready()
    {
        let (position, rotation) = crate::game::chetars_spatial_gui::anchor();
        let relative = glam::DVec3::from_array(position) - camera_position;
        let direction = if relative.length_squared() > 0.0001 {
            relative.normalize()
        } else {
            (current_rotation * glam::Vec3::NEG_Z).as_dvec3()
        };
        crate::game::chetars_spatial_gui::save_anchor(
            (camera_position + direction * distance as f64).to_array(),
            rotation,
        );
        let normal = relative.normalize_or_zero();
        if normal.length_squared() > 0.5 {
            crate::game::chetars_spatial_gui::set_orbit_angles(
                normal.x.atan2(-normal.z) as f32,
                normal.y.clamp(-1.0, 1.0).asin() as f32,
            );
        }
    }

    let panel_position = if follow {
        let (orbit_yaw, orbit_pitch) = crate::game::chetars_spatial_gui::orbit_angles();
        let world_offset = crate::game::chetars_spatial_gui::orbit_offset(
            orbit_yaw,
            orbit_pitch,
            distance,
        );
        camera_position + world_offset.as_dvec3()
    } else {
        let (position, _) = crate::game::chetars_spatial_gui::anchor();
        glam::DVec3::from_array(position)
    };

    // The face points toward the player while the panel is being orbited.
    // In Keep mode we save that final yaw on every drag frame, then freeze it
    // when RMB is released so the panel remains a fixed 3D object.
    let toward_player = (camera_position - panel_position).as_vec3().normalize_or_zero();
    let face_yaw = if toward_player.length_squared() > 0.5 {
        toward_player.x.atan2(toward_player.z)
    } else {
        camera_yaw
    };
    let facing_rotation = glam::Quat::from_rotation_y(face_yaw);
    let panel_rotation = if follow || crate::game::chetars_spatial_gui::is_dragging() {
        if !follow {
            crate::game::chetars_spatial_gui::save_anchor(
                panel_position.to_array(),
                facing_rotation,
            );
        }
        facing_rotation
    } else {
        crate::game::chetars_spatial_gui::anchor().1
    };

    let panel_width_pixels = (panel.body[1][0] - panel.body[0][0]).abs().max(1.0);
    let world_per_pixel = (3.6 / panel_width_pixels)
        .clamp(0.001, 0.02)
        * crate::game::chetars_spatial_gui::panel_scale();

    // Project the crosshair ray onto the panel plane. A back-side hit reverses
    // the local horizontal coordinate because the rear face is turned 180°.
    let ray_direction = current_rotation * glam::Vec3::NEG_Z;
    let panel_normal = panel_rotation * glam::Vec3::Z;
    let denominator = ray_direction.dot(panel_normal);
    let mut hit_depth = f32::INFINITY;
    let hit = if denominator.abs() > 0.0001 {
        let to_panel = panel_position - camera_position;
        let t = to_panel.dot(panel_normal.as_dvec3()) / denominator as f64;
        if t > 0.0 && t.is_finite() {
            hit_depth = t as f32;
            let world_hit = camera_position + ray_direction.as_dvec3() * t;
            let local_hit =
                panel_rotation.inverse() * (world_hit - panel_position).as_vec3();
            let local_x = if denominator > 0.0 { -local_hit.x } else { local_hit.x };
            Some([
                centre_x + local_x / world_per_pixel,
                centre_y - local_hit.y / world_per_pixel,
            ])
        } else {
            None
        }
    } else {
        None
    };

    // Only the nearest panel whose actual rectangle is under the crosshair owns input.
    let hit_inside = hit.is_some_and(|[screen_x, screen_y]| {
        screen_x >= panel.body[0][0]
            && screen_x <= panel.body[1][0]
            && screen_y >= panel.body[0][1]
            && screen_y <= panel.body[2][1]
    });
    let is_primary_hit = hit_inside
        && crate::game::chetars_spatial_gui::set_hovered_panel_if_closer(panel_index, hit_depth);
    if is_primary_hit {
        // If a nearer panel takes over the pointer, discard the previous
        // panel's resize hot zone before evaluating this face.
        crate::game::chetars_spatial_gui::set_hovered_resize_panel(None);
        if let Some([screen_x, screen_y]) = hit {
            let plus_x = panel.header[1][0] - 72.0;
            let close_x = plus_x + 40.0;
            let plus_y = panel.header[0][1] + 8.0;
            let over_add = crate::game::chetars_spatial_gui::panel_count()
                < crate::game::chetars_spatial_gui::max_panels()
                && screen_x >= plus_x && screen_x <= plus_x + 34.0
                && screen_y >= plus_y && screen_y <= plus_y + 30.0;
            let over_close = crate::game::chetars_spatial_gui::panel_count() > 1
                && screen_x >= close_x && screen_x <= close_x + 34.0
                && screen_y >= plus_y && screen_y <= plus_y + 30.0;
            crate::game::chetars_spatial_gui::set_hovered_add(over_add);
            let drag_hover = !over_add
                && screen_x >= panel.body[0][0]
                && screen_x <= panel.body[1][0]
                && screen_y >= panel.body[0][1]
                && screen_y <= panel.header[2][1];
            crate::game::chetars_spatial_gui::set_drag_hovered(drag_hover);

            let mut nav = None;
            for index in 0..3 {
                let button = panel.buttons[index];
                if screen_x >= button[0][0] && screen_x <= button[1][0]
                    && screen_y >= button[0][1] && screen_y <= button[2][1]
                {
                    nav = Some(index);
                    break;
                }
            }
            crate::game::chetars_spatial_gui::set_hovered_navigation(nav);

            let resize_size = 30.0;
            let over_resize = screen_x >= panel.body[0][0] + 3.0
                && screen_x <= panel.body[0][0] + resize_size
                && screen_y >= panel.body[0][1] + 3.0
                && screen_y <= panel.body[0][1] + resize_size;
            if over_resize {
                crate::game::chetars_spatial_gui::set_hovered_resize_panel(Some(panel_index));
            }

            let header_left = panel.header[0][0];
            let header_right = panel.header[1][0];
            let arrow_y = panel.header[0][1] + 9.0;
            let prev_x = header_left + 244.0;
            let next_x = header_left + 280.0;
            let over_prev = screen_x >= prev_x && screen_x <= prev_x + 28.0
                && screen_y >= arrow_y && screen_y <= arrow_y + 30.0;
            let over_next = screen_x >= next_x && screen_x <= next_x + 28.0
                && screen_y >= arrow_y && screen_y <= arrow_y + 30.0;

            let list_top = panel.content_frame[0][1] + 58.0;
            let list_bottom = panel.content_frame[2][1] - 10.0;
            let row_gap = 4.0;
            let row_h = ((list_bottom - list_top - row_gap * 9.0) / 10.0).clamp(22.0, 34.0);
            let list_left = panel.content_frame[0][0] + 8.0;
            let list_right = panel.content_frame[1][0] - 8.0;
            let mut action = if over_resize { 11u8 }
                else if over_prev { 8 }
                else if over_next { 9 }
                else { 0 };

            // Each visible row selects the corresponding independent 3D panel.
            let mut visible_row = 0usize;
            for target in 0..crate::game::chetars_spatial_gui::max_panels() {
                if !crate::game::chetars_spatial_gui::panel_exists(target) {
                    continue;
                }
                let row_y = list_top + visible_row as f32 * (row_h + row_gap);
                visible_row += 1;
                if screen_x >= list_left && screen_x <= list_right
                    && screen_y >= row_y && screen_y <= row_y + row_h
                {
                    action = 12 + target as u8;
                }
            }

            if crate::game::chetars_spatial_gui::selected_button() == 2 {
                let left = panel.white_area[0][0];
                let top = panel.white_area[0][1];
                let right = panel.white_area[1][0];
                let row_left = left + 12.0;
                let row_right = right - 12.0;

                let mode_y = top + 50.0;
                let mode_over = screen_x >= row_left && screen_x <= row_right
                    && screen_y >= mode_y && screen_y <= mode_y + 36.0;
                if mode_over { action = 1; }

                if crate::game::chetars_spatial_gui::dropdown_open() {
                    let option_y = top + 94.0;
                    if screen_x >= row_left + 8.0 && screen_x <= row_right - 8.0
                        && screen_y >= option_y && screen_y <= option_y + 30.0
                    { action = 2; }
                    else if screen_x >= row_left + 8.0 && screen_x <= row_right - 8.0
                        && screen_y >= option_y + 34.0 && screen_y <= option_y + 64.0
                    { action = 3; }
                }

                let two_d_y = top + 176.0;
                if screen_x >= row_left && screen_x <= row_right
                    && screen_y >= two_d_y && screen_y <= two_d_y + 36.0
                { action = 4; }

                let distance_y = top + 224.0;
                let minus_x = right - 100.0;
                let plus_x = right - 54.0;
                if screen_y >= distance_y && screen_y <= distance_y + 38.0
                    && screen_x >= minus_x && screen_x <= minus_x + 38.0
                { action = 5; }
                else if screen_y >= distance_y && screen_y <= distance_y + 38.0
                    && screen_x >= plus_x && screen_x <= plus_x + 38.0
                { action = 6; }
            }

            let add_x = header_right - 112.0;
            let add_y = panel.header[0][1] + 9.0;
            let over_add = crate::game::chetars_spatial_gui::panel_count()
                < crate::game::chetars_spatial_gui::max_panels()
                && screen_x >= add_x && screen_x <= add_x + 34.0
                && screen_y >= add_y && screen_y <= add_y + 30.0;
            let close_x = add_x + 40.0;
            let over_close = crate::game::chetars_spatial_gui::panel_count() > 1
                && screen_x >= close_x && screen_x <= close_x + 34.0
                && screen_y >= add_y && screen_y <= add_y + 30.0;
            crate::game::chetars_spatial_gui::set_hovered_add(over_add);
            crate::game::chetars_spatial_gui::set_hovered_mode_action(
                if over_resize { 11 }
                else if over_close { 10 }
                else if over_add { 7 }
                else { action }
            );
        }
    }

    let paint = |game: &crate::game::Game<'_>,
                 draw: &mut NativeWorldDraw,
                 points: [[f32; 3]; 4],
                 argb: u32| {
        let corners = [
            [points[0][0] - centre_x, centre_y - points[0][1], points[0][2]],
            [points[3][0] - centre_x, centre_y - points[3][1], points[3][2]],
            [points[2][0] - centre_x, centre_y - points[2][1], points[2][2]],
            [points[1][0] - centre_x, centre_y - points[1][1], points[1][2]],
        ];
        let rgba = [
            ((argb >> 16) & 0xFF) as u8,
            ((argb >> 8) & 0xFF) as u8,
            (argb & 0xFF) as u8,
            ((argb >> 24) & 0xFF) as u8,
        ];
        let _ = game.native_world_quad(
            draw,
            "horizon/white",
            corners,
            [0.0, 0.0, 16.0, 16.0],
            rgba,
        );
    };

    let rect = |x: f32, y: f32, w: f32, h: f32, z: f32| {
        [
            [x, y, z],
            [x + w, y, z],
            [x + w, y + h, z],
            [x, y + h, z],
        ]
    };

    let label = |game: &crate::game::Game<'_>,
                 draw: &mut NativeWorldDraw,
                 x: f32,
                 y: f32,
                 text: &str,
                 color: u32| {
        draw.push();
        draw.translate([x - centre_x, centre_y - y, 14.0]);
        draw.scale([1.0, -1.0, 1.0]);
        let text = crate::text::Text::literal(text);
        let _ = game.native_world_text(draw, &text, color, None);
        draw.pop();
    };

    let draw_face = |game: &crate::game::Game<'_>, draw: &mut NativeWorldDraw| {
        let bg = 0xFF141A2A;
        let card = 0xFF242D43;
        let card_alt = 0xFF2C3650;
        let card_hover = 0xFF3C4567;
        let purple = 0xFF7C5CFF;
        let blue = 0xFF6EA8FF;
        let bright = 0xFFB8C2FF;
        let muted = 0xFF8794BA;
        let white = 0xFFF2F4FF;

        paint(game, draw, panel.shadow, 0xAA000000);
        let side_faces = [
            [panel.body[0], panel.body[1], panel.shadow[1], panel.shadow[0]],
            [panel.body[1], panel.body[2], panel.shadow[2], panel.shadow[1]],
            [panel.body[2], panel.body[3], panel.shadow[3], panel.shadow[2]],
            [panel.body[3], panel.body[0], panel.shadow[0], panel.shadow[3]],
        ];
        for side in side_faces { paint(game, draw, side, 0xFF0B1020); }

        paint(game, draw, panel.body, bg);
        paint(game, draw, panel.inner, 0xFF1B2336);
        paint(game, draw, panel.header, 0xFF202940);
        paint(game, draw, panel.sidebar, 0xFF202A40);
        paint(game, draw, panel.content_frame, 0xFF202A40);
        paint(game, draw, panel.white_area, 0xFF242D43);
        paint(game, draw, panel.info_frame, 0xFF202A40);
        paint(game, draw, panel.accent_left, purple);
        paint(game, draw, panel.accent_top, blue);

        // Dotted triangular grip remains in the same top-left position.
        let resize_color = if crate::game::chetars_spatial_gui::hovered_resize_panel() == Some(panel_index)
            || crate::game::chetars_spatial_gui::is_3d_resizing_for(panel_index)
        { blue } else { bright };
        for row in 0..5 {
            for col in 0..=row {
                paint(game, draw, rect(
                    panel.body[0][0] + 9.0 + col as f32 * 4.2,
                    panel.body[0][1] + 9.0 + row as f32 * 4.2,
                    2.2, 2.2, 13.0,
                ), resize_color);
            }
        }

        // Header.
        paint(game, draw, rect(panel.body[0][0] + 18.0, panel.body[0][1] + 15.0, 28.0, 28.0, 8.0), purple);
        paint(game, draw, rect(panel.body[0][0] + 33.0, panel.body[0][1] + 15.0, 13.0, 28.0, 9.0), blue);
        label(game, draw, panel.body[0][0] + 54.0, panel.body[0][1] + 24.0, "CHETARS", white);
        label(game, draw, panel.body[0][0] + 116.0, panel.body[0][1] + 24.0, "SPATIAL CONTROL CENTER", muted);

        let header_left = panel.header[0][0];
        let header_right = panel.header[1][0];
        let arrow_y = panel.header[0][1] + 9.0;
        let prev_x = header_left + 244.0;
        let next_x = header_left + 280.0;
        for (bx, symbol, action) in [(prev_x, "<", 8u8), (next_x, ">", 9u8)] {
            let hovered = crate::game::chetars_spatial_gui::hovered_mode_action() == action;
            paint(game, draw, rect(bx, arrow_y, 28.0, 30.0, 11.0),
                if hovered { card_hover } else { card_alt });
            label(game, draw, bx + 10.0, arrow_y + 20.0, symbol, white);
        }

        let add_x = header_right - 112.0;
        let add_y = panel.header[0][1] + 9.0;
        let can_add = crate::game::chetars_spatial_gui::panel_count()
            < crate::game::chetars_spatial_gui::max_panels();
        paint(game, draw, rect(add_x, add_y, 34.0, 30.0, 11.0),
            if !can_add { 0xFF30394D }
            else if crate::game::chetars_spatial_gui::hovered_add() { purple }
            else { card_alt });
        label(game, draw, add_x + 12.0, add_y + 20.0, "+", white);
        let close_x = add_x + 40.0;
        paint(game, draw, rect(close_x, add_y, 34.0, 30.0, 11.0),
            if crate::game::chetars_spatial_gui::hovered_mode_action() == 10 { 0xFFB84B66 } else { card_alt });
        label(game, draw, close_x + 12.0, add_y + 20.0, "x", white);

        label(game, draw, panel.body[0][0] + 22.0, panel.body[0][1] + 64.0,
            &format!("PANEL {} / {}", panel_index + 1, crate::game::chetars_spatial_gui::panel_count()), muted);

        // Left navigation rail.
        for (i, name) in ["OVERVIEW", "PROFILE", "SETTINGS"].iter().enumerate() {
            let bx = panel.buttons[i][0][0];
            let by = panel.buttons[i][0][1];
            let active = crate::game::chetars_spatial_gui::selected_button() == i;
            let hovered = crate::game::chetars_spatial_gui::hovered_navigation() == Some(i);
            paint(game, draw, panel.buttons[i],
                if active { 0xFF514C86 } else if hovered { card_hover } else { card });
            if active {
                paint(game, draw, rect(bx, by, 3.0, 38.0, 10.0), blue);
            }
            label(game, draw, bx + 12.0, by + 23.0, ["01", "02", "03"][i],
                if active { blue } else { muted });
            label(game, draw, bx + 40.0, by + 23.0, name,
                if active || hovered { white } else { bright });
        }
        label(game, draw, panel.sidebar[0][0] + 12.0, panel.sidebar[3][1] - 28.0,
            "WORLD UI", muted);

        // Panel list column.
        let list_x = panel.content_frame[0][0];
        let list_y = panel.content_frame[0][1];
        let list_w = panel.content_frame[1][0] - list_x;
        label(game, draw, list_x + 12.0, list_y + 23.0, "SPATIAL PANELS", white);
        label(game, draw, list_x + 12.0, list_y + 40.0, "Select one to edit", muted);
        let rows_top = list_y + 58.0;
        let rows_bottom = panel.content_frame[2][1] - 10.0;
        let row_gap = 4.0;
        let row_h = ((rows_bottom - rows_top - row_gap * 9.0) / 10.0).clamp(22.0, 34.0);
        let mut visible_row = 0usize;
        for target in 0..crate::game::chetars_spatial_gui::max_panels() {
            if !crate::game::chetars_spatial_gui::panel_exists(target) { continue; }
            let ry = rows_top + visible_row as f32 * (row_h + row_gap);
            visible_row += 1;
            let selected = crate::game::chetars_spatial_gui::selected_panel() as usize == target;
            let hovered = crate::game::chetars_spatial_gui::hovered_panel() == Some(panel_index)
                && crate::game::chetars_spatial_gui::hovered_mode_action() == 12 + target as u8;
            paint(game, draw, rect(list_x + 8.0, ry, list_w - 16.0, row_h, 9.0),
                if selected { 0xFF454A78 } else if hovered { card_hover } else { card_alt });
            paint(game, draw, rect(list_x + 8.0, ry, 3.0, row_h, 10.0),
                if selected { blue } else { purple });
            label(game, draw, list_x + 18.0, ry + row_h * 0.62,
                &format!("PANEL {:02}", target + 1),
                if selected { white } else { bright });
            let mode = if crate::game::chetars_spatial_gui::selected_panel() as usize == target {
                if crate::game::chetars_spatial_gui::follow_mode() { "FOLLOW" } else { "KEEP" }
            } else { "READY" };
            label(game, draw, list_x + list_w - 58.0, ry + row_h * 0.62, mode, muted);
        }

        // Main content column.
        let settings_left = panel.white_area[0][0];
        let settings_top = panel.white_area[0][1];
        let settings_right = panel.white_area[1][0];
        let settings_w = settings_right - settings_left;
        let row_left = settings_left + 12.0;
        let row_w = settings_w - 24.0;
        let follow = crate::game::chetars_spatial_gui::follow_mode();
        let mode_label = if follow { "FOLLOW" } else { "KEEP" };
        let distance_label = format!("{:.1} BLOCKS", crate::game::chetars_spatial_gui::panel_distance());
        let scale_label = format!("{:.0}%", crate::game::chetars_spatial_gui::panel_scale() * 100.0);

        match crate::game::chetars_spatial_gui::selected_button() {
            0 => {
                label(game, draw, settings_left + 14.0, settings_top + 24.0, "OVERVIEW", white);
                label(game, draw, settings_left + 14.0, settings_top + 42.0, "Your spatial workspace", muted);
                let cards = [
                    (settings_top + 64.0, "3D WORLD PANELS", "Floating displays in your world"),
                    (settings_top + 124.0, "360 ORBIT CONTROL", "Hold RMB on a panel header"),
                    (settings_top + 184.0, "INDEPENDENT PANELS", "Each panel keeps its own position"),
                ];
                for (cy, title, detail) in cards {
                    paint(game, draw, rect(row_left, cy, row_w, 48.0, 9.0), card_alt);
                    paint(game, draw, rect(row_left, cy, 3.0, 48.0, 10.0), purple);
                    label(game, draw, row_left + 12.0, cy + 17.0, title, white);
                    label(game, draw, row_left + 12.0, cy + 35.0, detail, muted);
                }
                label(game, draw, row_left, settings_top + 270.0,
                    &format!("ACTIVE PANELS: {} / {}", crate::game::chetars_spatial_gui::panel_count(), crate::game::chetars_spatial_gui::max_panels()), bright);
            }
            1 => {
                label(game, draw, settings_left + 14.0, settings_top + 24.0, "PANEL PROFILE", white);
                label(game, draw, settings_left + 14.0, settings_top + 42.0, "Selected display details", muted);
                let cards = [
                    (settings_top + 68.0, "PLACEMENT MODE", mode_label),
                    (settings_top + 132.0, "PANEL DISTANCE", distance_label.as_str()),
                    (settings_top + 196.0, "PANEL SCALE", scale_label.as_str()),
                ];
                for (cy, title, value) in cards {
                    paint(game, draw, rect(row_left, cy, row_w, 48.0, 9.0), card_alt);
                    label(game, draw, row_left + 12.0, cy + 17.0, title, muted);
                    label(game, draw, row_left + 12.0, cy + 36.0, value, white);
                }
                label(game, draw, row_left, settings_top + 280.0,
                    "RMB ORBIT KEEPS THE PANEL UPRIGHT", bright);
            }
            _ => {
                label(game, draw, settings_left + 14.0, settings_top + 24.0, "DISPLAY SETTINGS", white);
                label(game, draw, settings_left + 14.0, settings_top + 42.0, "Configure the selected panel", muted);

                let mode_y = settings_top + 50.0;
                let mode_open = crate::game::chetars_spatial_gui::dropdown_open();
                paint(game, draw, rect(row_left, mode_y, row_w, 36.0, 9.0),
                    if mode_open { 0xFF3A4564 } else { card_alt });
                label(game, draw, row_left + 12.0, mode_y + 23.0, "FOLLOW / KEEP", bright);
                label(game, draw, row_left + row_w - 88.0, mode_y + 23.0, mode_label, blue);
                label(game, draw, row_left + row_w - 18.0, mode_y + 23.0,
                    if mode_open { "^" } else { "v" }, white);

                if mode_open {
                    let option_y = settings_top + 94.0;
                    paint(game, draw, rect(row_left + 8.0, option_y, row_w - 16.0, 30.0, 10.0),
                        if follow { 0xFF514C86 } else { card });
                    label(game, draw, row_left + 20.0, option_y + 20.0, "FOLLOW", white);
                    paint(game, draw, rect(row_left + 8.0, option_y + 34.0, row_w - 16.0, 30.0, 10.0),
                        if !follow { 0xFF514C86 } else { card });
                    label(game, draw, row_left + 20.0, option_y + 54.0, "KEEP AT COORDINATES", white);
                }

                let two_d_y = settings_top + 176.0;
                let two_d = crate::game::chetars_spatial_gui::is_2d_mode();
                paint(game, draw, rect(row_left, two_d_y, row_w, 36.0, 9.0), card_alt);
                label(game, draw, row_left + 12.0, two_d_y + 23.0, "2D OVERLAY", bright);
                let sw_x = row_left + row_w - 58.0;
                paint(game, draw, rect(sw_x, two_d_y + 5.0, 46.0, 26.0, 12.0),
                    if two_d { purple } else { 0xFF4A5367 });
                paint(game, draw, rect(sw_x + if two_d { 23.0 } else { 3.0 }, two_d_y + 8.0, 20.0, 20.0, 13.0), white);

                let distance_y = settings_top + 224.0;
                paint(game, draw, rect(row_left, distance_y, row_w, 38.0, 9.0), card_alt);
                label(game, draw, row_left + 12.0, distance_y + 24.0, "DISTANCE", bright);
                label(game, draw, row_left + row_w - 148.0, distance_y + 24.0,
                    &format!("{:.1}", crate::game::chetars_spatial_gui::panel_distance()), white);
                let minus_x = settings_right - 100.0;
                let plus_x = settings_right - 54.0;
                paint(game, draw, rect(minus_x, distance_y + 3.0, 38.0, 32.0, 11.0), card);
                paint(game, draw, rect(plus_x, distance_y + 3.0, 38.0, 32.0, 11.0), card);
                label(game, draw, minus_x + 15.0, distance_y + 23.0, "-", white);
                label(game, draw, plus_x + 15.0, distance_y + 23.0, "+", white);
                label(game, draw, row_left, settings_top + 288.0,
                    "RESIZE WITH THE DOTTED CORNER GRIP", muted);
            }
        }

        // Real status/shortcuts only; no fictional Configs or combat modules.
        let info_x = panel.info_frame[0][0];
        let info_y = panel.info_frame[0][1];
        let info_w = panel.info_frame[1][0] - info_x;
        paint(game, draw, rect(info_x + 8.0, info_y + 10.0, info_w - 16.0, 150.0, 9.0), card);
        label(game, draw, info_x + 18.0, info_y + 32.0, "PANEL STATUS", white);
        label(game, draw, info_x + 18.0, info_y + 60.0,
            &format!("{:02} / 10 ACTIVE", panel_index + 1), blue);
        label(game, draw, info_x + 18.0, info_y + 86.0,
            if follow { "FOLLOW MODE" } else { "KEEP MODE" }, bright);
        label(game, draw, info_x + 18.0, info_y + 112.0, distance_label.as_str(), muted);
        label(game, draw, info_x + 18.0, info_y + 136.0, &format!("SCALE {}", scale_label), muted);

        let quick_y = info_y + 172.0;
        let quick_h = (panel.info_frame[3][1] - quick_y - 10.0).max(104.0);
        paint(game, draw, rect(info_x + 8.0, quick_y, info_w - 16.0, quick_h, 9.0), card);
        label(game, draw, info_x + 18.0, quick_y + 22.0, "QUICK CONTROLS", white);
        for (i, (key, name)) in [
            ("F6", "OPEN / CLOSE"),
            ("F7", "2D / 3D"),
            ("RMB", "ORBIT"),
            ("LMB", "RESIZE"),
            ("WHEEL", "DISTANCE"),
        ].iter().enumerate() {
            let ky = quick_y + 48.0 + i as f32 * 32.0;
            label(game, draw, info_x + 18.0, ky, key, blue);
            label(game, draw, info_x + 66.0, ky, name, bright);
        }
    };

    draw.push();
    let _ = draw.translate_world(panel_position.to_array());
    let (panel_yaw, panel_pitch, panel_roll) =
        panel_rotation.to_euler(glam::EulerRot::YXZ);
    draw.rotate_about([0.0, 1.0, 0.0], panel_yaw.to_degrees(), [0.0; 3]);
    draw.rotate_about([1.0, 0.0, 0.0], panel_pitch.to_degrees(), [0.0; 3]);
    draw.rotate_about([0.0, 0.0, 1.0], panel_roll.to_degrees(), [0.0; 3]);
    draw.scale([world_per_pixel, world_per_pixel, world_per_pixel]);

    draw_face(game, draw);
    draw.push();
    draw.rotate_about([0.0, 1.0, 0.0], 180.0, [0.0; 3]);
    draw_face(game, draw);
    draw.pop();
    draw.pop();
    };


    if !game.native_playing() || !crate::game::chetars_spatial_gui::is_open() {
        crate::game::chetars_spatial_gui::clear_hover_state();
        return;
    }

    crate::game::chetars_spatial_gui::clear_hover_state();
    for index in 0..crate::game::chetars_spatial_gui::max_panels() {
        if !crate::game::chetars_spatial_gui::panel_exists(index) {
            continue;
        }
        crate::game::chetars_spatial_gui::load_panel(index);
        draw_panel(game, draw, index);
        crate::game::chetars_spatial_gui::save_panel(index);
    }
    crate::game::chetars_spatial_gui::load_panel(
        crate::game::chetars_spatial_gui::selected_panel() as usize,
    );

}
