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

            let left = panel.white_area[0][0];
            let top = panel.white_area[0][1];
            let right = panel.white_area[1][0];
            let mode_y = top + 26.0;
            let row_left = left + 22.0;
            let row_right = right - 22.0;
            let resize_size = 30.0;
            let over_resize = screen_x >= panel.body[0][0] + 3.0
                && screen_x <= panel.body[0][0] + resize_size
                && screen_y >= panel.body[0][1] + 3.0
                && screen_y <= panel.body[0][1] + resize_size;
                if over_resize {
                crate::game::chetars_spatial_gui::set_hovered_resize_panel(Some(panel_index));
            }

            let mut action = if over_resize { 11u8 } else { 0u8 };
            if crate::game::chetars_spatial_gui::selected_button() == 2 {
                if screen_x >= row_left && screen_x <= row_right
                    && screen_y >= mode_y && screen_y <= mode_y + 38.0
                {
                    action = 1;
                }

                if crate::game::chetars_spatial_gui::dropdown_open() {
                    let option_y = top + 70.0;
                    if screen_x >= row_left && screen_x <= row_right
                        && screen_y >= option_y && screen_y <= option_y + 32.0
                    { action = 2; }
                    else if screen_x >= row_left && screen_x <= row_right
                        && screen_y >= option_y + 36.0 && screen_y <= option_y + 68.0
                    { action = 3; }
                }

                let two_d_y = top + 148.0;
                if screen_x >= row_left && screen_x <= row_right
                    && screen_y >= two_d_y && screen_y <= two_d_y + 34.0
                { action = 4; }

                let distance_y = top + 192.0;
                let minus_x = right - 112.0;
                let plus_x = right - 58.0;
                if screen_y >= distance_y && screen_y <= distance_y + 36.0
                    && screen_x >= minus_x && screen_x <= minus_x + 42.0
                { action = 5; }
                else if screen_y >= distance_y && screen_y <= distance_y + 36.0
                    && screen_x >= plus_x && screen_x <= plus_x + 42.0
                { action = 6; }
            }
            crate::game::chetars_spatial_gui::set_hovered_mode_action(
                if over_close { 10 } else if over_add { 7 } else { action }
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
        paint(game, draw, panel.shadow, 0x99000000);
        let side_faces = [
            [panel.body[0], panel.body[1], panel.shadow[1], panel.shadow[0]],
            [panel.body[1], panel.body[2], panel.shadow[2], panel.shadow[1]],
            [panel.body[2], panel.body[3], panel.shadow[3], panel.shadow[2]],
            [panel.body[3], panel.body[0], panel.shadow[0], panel.shadow[3]],
        ];
        for side in side_faces {
            paint(game, draw, side, 0xFF080A0D);
        }

        paint(game, draw, panel.body, 0xFF101010);
        paint(game, draw, panel.inner, 0xFF1E1E1E);
        paint(game, draw, panel.header, 0xFF101010);

        // Dotted triangular handle at the upper-left corner resizes this 3D panel.
        let resize_color = if crate::game::chetars_spatial_gui::hovered_resize_panel() == Some(panel_index)
            || crate::game::chetars_spatial_gui::is_3d_resizing_for(panel_index)
        { 0xFF55C7FF } else { 0xFFB8C5D1 };
        for row in 0..5 {
            for col in 0..=row {
                paint(
                    game,
                    draw,
                    rect(
                        panel.body[0][0] + 9.0 + col as f32 * 4.2,
                        panel.body[0][1] + 9.0 + row as f32 * 4.2,
                        2.2,
                        2.2,
                        13.0,
                    ),
                    resize_color,
                );
            }
        }

        let plus_x = panel.header[1][0] - 72.0;
        let close_x = plus_x + 40.0;
        let plus_y = panel.header[0][1] + 8.0;
        let can_add = crate::game::chetars_spatial_gui::panel_count()
            < crate::game::chetars_spatial_gui::max_panels();
        paint(
            game,
            draw,
            rect(plus_x, plus_y, 34.0, 30.0, 12.0),
            if !can_add { 0xFF272D34 }
            else if crate::game::chetars_spatial_gui::hovered_panel() == Some(panel_index)
                && crate::game::chetars_spatial_gui::hovered_add()
            { 0xFF33AAFF } else { 0xFF303844 },
        );
        label(game, draw, plus_x + 12.0, plus_y + 21.0, "+", 0xFFFFFFFF);
        paint(
            game,
            draw,
            rect(close_x, plus_y, 34.0, 30.0, 12.0),
            if crate::game::chetars_spatial_gui::hovered_panel() == Some(panel_index)
                && crate::game::chetars_spatial_gui::hovered_mode_action() == 10
            { 0xFFCC3344 } else { 0xFF303844 },
        );
        label(game, draw, close_x + 11.0, plus_y + 21.0, "x", 0xFFFFFFFF);
        paint(game, draw, panel.sidebar, 0xFF1E1E1E);
        paint(game, draw, panel.content_frame, 0xFF4A4A4A);
        paint(game, draw, panel.white_area, 0xFFFFFFFF);

        for i in 0..3 {
            let color = if crate::game::chetars_spatial_gui::selected_button() == i {
                0xFF33AAFF
            } else {
                0xFF2C2C2C
            };
            paint(game, draw, panel.buttons[i], color);
        }
        paint(game, draw, panel.accent_left, 0xFF33AAFF);
        paint(game, draw, panel.accent_top, 0xFF33AAFF);

        label(game, draw, panel.body[0][0] + 48.0, panel.body[0][1] + 22.0, "CHETARS", 0xFFFFFFFF);
        label(
            game,
            draw,
            panel.body[0][0] + 110.0,
            panel.body[0][1] + 24.0,
            &format!(
                "PANEL {} / {}",
                panel_index + 1,
                crate::game::chetars_spatial_gui::panel_count()
            ),
            0xFFCCCCCC,
        );

        for (i, text) in ["HOME", "PROFILE", "SETTINGS"].iter().enumerate() {
            let x = panel.buttons[i][0][0] + 14.0;
            let y = panel.buttons[i][0][1] + 12.0;
            label(
                game,
                draw,
                x,
                y,
                text,
                if crate::game::chetars_spatial_gui::selected_button() == i {
                    0xFFFFFFFF
                } else if crate::game::chetars_spatial_gui::hovered_navigation() == Some(i) {
                    0xFF33AAFF
                } else {
                    0xFFCCCCCC
                },
            );
        }

        let left = panel.white_area[0][0];
        let top = panel.white_area[0][1];
        let right = panel.white_area[1][0];
        let row_x = left + 22.0;
        let row_width = (right - left - 44.0).max(120.0);
        let mode = if crate::game::chetars_spatial_gui::follow_mode() {
            "FOLLOW ME"
        } else {
            "KEEP AT COORDINATES"
        };
        let distance_text = format!(
            "{:.1} BLOCKS",
            crate::game::chetars_spatial_gui::panel_distance()
        );

        match crate::game::chetars_spatial_gui::selected_button() {
            0 => {
                // HOME is now a usable function catalogue instead of an empty
                // white rectangle.
                label(game, draw, row_x, top + 16.0, "FUNCTION CATALOG", 0xFF303844);
                let cards = [
                    (
                        top + 38.0,
                        "3D WORLD DISPLAY",
                        "Floating panel, independent from the inventory",
                    ),
                    (
                        top + 98.0,
                        "ORBIT DRAG",
                        "Hold RMB on the title bar and turn around",
                    ),
                    (
                        top + 158.0,
                        "DISTANCE CONTROL",
                        "Scroll on the title bar or use Settings + / -",
                    ),
                ];
                for (y, title, detail) in cards {
                    paint(game, draw, rect(row_x, y, row_width, 48.0, 9.0), 0xFFF2F4F7);
                    paint(game, draw, rect(row_x, y, 3.0, 48.0, 10.0), 0xFF33AAFF);
                    label(game, draw, row_x + 12.0, y + 14.0, title, 0xFF202833);
                    label(game, draw, row_x + 12.0, y + 32.0, detail, 0xFF687482);
                }
            }
            1 => {
                // PROFILE reports the active placement configuration and
                // the controls that affect this panel.
                label(game, draw, row_x, top + 16.0, "PANEL PROFILE", 0xFF303844);
                let cards = [
                    (top + 42.0, "PLACEMENT MODE", mode),
                    (top + 104.0, "CURRENT DISTANCE", distance_text.as_str()),
                    (top + 166.0, "MOVEMENT", "RMB drag orbits around the player"),
                ];
                for (y, title, detail) in cards {
                    paint(game, draw, rect(row_x, y, row_width, 48.0, 9.0), 0xFFF2F4F7);
                    paint(game, draw, rect(row_x, y, 3.0, 48.0, 10.0), 0xFF33AAFF);
                    label(game, draw, row_x + 12.0, y + 14.0, title, 0xFF202833);
                    label(game, draw, row_x + 12.0, y + 32.0, detail, 0xFF687482);
                }
            }
            2 => {
                let mode_y = top + 26.0;
                let mode_text = if crate::game::chetars_spatial_gui::follow_mode() {
                    "FOLLOW"
                } else {
                    "KEEP"
                };
                let open = crate::game::chetars_spatial_gui::dropdown_open();
                paint(game, draw, rect(row_x, mode_y, row_width, 38.0, 9.0),
                    if open { 0xFFEAF5FF } else { 0xFFF2F4F7 });
                label(game, draw, row_x + 12.0, mode_y + 24.0, "FOLLOW / KEEP", 0xFF202833);
                label(game, draw, row_x + row_width - 118.0, mode_y + 24.0, mode_text, 0xFF303844);
                label(game, draw, row_x + row_width - 34.0, mode_y + 24.0,
                    if open { "^" } else { "v" }, 0xFF303844);

                if open {
                    let option_y = top + 70.0;
                    let follow_color = if crate::game::chetars_spatial_gui::follow_mode() { 0xFFDCEEFF } else { 0xFFF2F4F7 };
                    let keep_color = if !crate::game::chetars_spatial_gui::follow_mode() { 0xFFDCEEFF } else { 0xFFF2F4F7 };
                    paint(game, draw, rect(row_x + 16.0, option_y, row_width - 16.0, 32.0, 9.0), follow_color);
                    label(game, draw, row_x + 28.0, option_y + 21.0, "FOLLOW", 0xFF202833);
                    paint(game, draw, rect(row_x + 16.0, option_y + 36.0, row_width - 16.0, 32.0, 9.0), keep_color);
                    label(game, draw, row_x + 28.0, option_y + 57.0, "KEEP", 0xFF202833);
                }

                let two_d_y = top + 148.0;
                let two_d = crate::game::chetars_spatial_gui::is_2d_mode();
                let two_progress = crate::game::chetars_spatial_gui::two_d_progress();
                paint(game, draw, rect(row_x, two_d_y, row_width, 34.0, 9.0), 0xFFF2F4F7);
                label(game, draw, row_x + 12.0, two_d_y + 22.0, "2D PANEL", 0xFF202833);

                // Independent presentation switch; it does not alter Follow / Keep.
                let switch_x = row_x + row_width - 66.0;
                let switch_y = two_d_y + 4.0;
                let switch_w = 50.0;
                let switch_h = 26.0;
                paint(game, draw, rect(switch_x, switch_y, switch_w, switch_h, 13.0),
                    if two_d { 0xFF167D48 } else { 0xFF8A949E });
                let knob_x = switch_x + 3.0 + two_progress * (switch_w - 20.0);
                paint(game, draw, rect(knob_x, switch_y + 3.0, 20.0, 20.0, 10.0), 0xFFFFFFFF);

                let distance_y = top + 192.0;
                paint(game, draw, rect(row_x, distance_y, row_width, 36.0, 9.0), 0xFFF2F4F7);
                label(game, draw, row_x + 12.0, distance_y + 23.0, "DISTANCE", 0xFF202833);
                let distance_text = format!("{:.1}", crate::game::chetars_spatial_gui::panel_distance());
                label(game, draw, row_x + row_width - 156.0, distance_y + 23.0, distance_text.as_str(), 0xFF303844);
                let minus_x = right - 112.0;
                let plus_x = right - 58.0;
                paint(game, draw, rect(minus_x, distance_y + 2.0, 42.0, 32.0, 11.0), 0xFF4B5563);
                paint(game, draw, rect(plus_x, distance_y + 2.0, 42.0, 32.0, 11.0), 0xFF4B5563);
                label(game, draw, minus_x + 15.0, distance_y + 23.0, "-", 0xFFFFFFFF);
                label(game, draw, plus_x + 14.0, distance_y + 23.0, "+", 0xFFFFFFFF);
            }
            _ => {}
        }

        label(
            game,
            draw,
            panel.body[3][0] + 22.0,
            panel.body[3][1] - 22.0,
            "AIM AT A CONTROL + LEFT CLICK  |  RMB ON TOP BAR TO ORBIT",
            0xFFCCCCCC,
        );
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
