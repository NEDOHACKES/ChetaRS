fn world_submit(
    game: &crate::game::Game<'_>,
    draw: &mut NativeWorldDraw,
) {
    if !game.native_playing() || !crate::game::chetars_spatial_gui::is_open() {
        crate::game::chetars_spatial_gui::set_drag_hovered(false);
        crate::game::chetars_spatial_gui::set_hovered_setting(None);
        crate::game::chetars_spatial_gui::set_hovered_navigation(None);
        crate::game::chetars_spatial_gui::set_hovered_distance_control(None);
        return;
    }

    let (width, height, mouse) = crate::game::chetars_spatial_gui::viewport();
    if width <= 0.0 || height <= 0.0 {
        crate::game::chetars_spatial_gui::set_drag_hovered(false);
        crate::game::chetars_spatial_gui::set_hovered_setting(None);
        crate::game::chetars_spatial_gui::set_hovered_navigation(None);
        crate::game::chetars_spatial_gui::set_hovered_distance_control(None);
        return;
    }

    let panel = crate::game::chetars_spatial_gui::panel(mouse, width, height);
    let centre_x = width * 0.5;
    let centre_y = height * 0.5;
    let current_rotation = game.camera_orientation();
    let (camera_yaw, camera_pitch, _) = current_rotation.to_euler(glam::EulerRot::YXZ);
    let camera_position = glam::DVec3::from_array(draw.camera);

    crate::game::chetars_spatial_gui::update_drag_view(
        camera_yaw,
        camera_pitch,
        camera_position.to_array(),
    );

    let follow = crate::game::chetars_spatial_gui::follow_mode();
    let distance_dirty = crate::game::chetars_spatial_gui::take_distance_dirty();
    let distance = crate::game::chetars_spatial_gui::panel_distance();

    if !crate::game::chetars_spatial_gui::anchor_ready() {
        let forward = current_rotation * glam::Vec3::NEG_Z;
        let direction = forward.normalize();
        let anchor_position = camera_position + direction.as_dvec3() * distance as f64;
        crate::game::chetars_spatial_gui::save_anchor(
            anchor_position.to_array(),
            current_rotation,
        );
        crate::game::chetars_spatial_gui::set_orbit_angles(
            direction.x.atan2(-direction.z),
            direction.y.clamp(-1.0, 1.0).asin(),
        );
    }

    // A wheel/button distance change moves a pinned panel along its current
    // camera-to-panel direction. While dragging, the orbit update already
    // applies the new radius, so no second reposition is needed.
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
    }

    let (panel_position, panel_rotation) = if follow {
        let (orbit_yaw, orbit_pitch) = crate::game::chetars_spatial_gui::orbit_angles();
        let world_offset = crate::game::chetars_spatial_gui::orbit_offset(
            orbit_yaw,
            orbit_pitch,
            distance,
        );
        let (_, fixed_rotation) = crate::game::chetars_spatial_gui::anchor();
        (
            camera_position + world_offset.as_dvec3(),
            fixed_rotation,
        )
    } else {
        let (position, rotation) = crate::game::chetars_spatial_gui::anchor();
        (glam::DVec3::from_array(position), rotation)
    };

    let panel_width_pixels = (panel.body[1][0] - panel.body[0][0]).abs().max(1.0);
    let world_per_pixel = (3.6 / panel_width_pixels).clamp(0.001, 0.02);

    // Project the crosshair ray onto the panel plane. A back-side hit reverses
    // the local horizontal coordinate because the rear face is turned 180°.
    let ray_direction = current_rotation * glam::Vec3::NEG_Z;
    let panel_normal = panel_rotation * glam::Vec3::Z;
    let denominator = ray_direction.dot(panel_normal);
    let hit = if denominator.abs() > 0.0001 {
        let to_panel = panel_position - camera_position;
        let t = to_panel.dot(panel_normal.as_dvec3()) / denominator as f64;
        if t > 0.0 && t.is_finite() {
            let world_hit = camera_position + ray_direction.as_dvec3() * t;
            let local_hit =
                panel_rotation.inverse() * (world_hit - panel_position).as_vec3();
            let local_x = if denominator > 0.0 {
                -local_hit.x
            } else {
                local_hit.x
            };
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

    // RMB dragging starts only from the top title bar.
    let drag_hover = hit.is_some_and(|[screen_x, screen_y]| {
        screen_x >= panel.body[0][0]
            && screen_x <= panel.body[1][0]
            && screen_y >= panel.body[0][1]
            && screen_y <= panel.header[2][1]
    });
    crate::game::chetars_spatial_gui::set_drag_hovered(drag_hover);

    // Sidebar navigation gets its own hit targets; this was previously drawn
    // but had no click handler, which made the three sections appear inert.
    let mut hovered_navigation = None;
    if let Some([screen_x, screen_y]) = hit {
        for index in 0..3 {
            let button = panel.buttons[index];
            if screen_x >= button[0][0]
                && screen_x <= button[1][0]
                && screen_y >= button[0][1]
                && screen_y <= button[2][1]
            {
                hovered_navigation = Some(index);
                break;
            }
        }
    }
    crate::game::chetars_spatial_gui::set_hovered_navigation(hovered_navigation);

    let mut hovered_setting = None;
    let mut hovered_distance = None;
    if crate::game::chetars_spatial_gui::selected_button() == 2 {
        if let Some([screen_x, screen_y]) = hit {
            let left = panel.white_area[0][0];
            let top = panel.white_area[0][1];
            let right = panel.white_area[1][0];
            let row_left = left + 22.0;
            let row_right = right - 22.0;
            let row1_top = top + 30.0;
            let row2_top = row1_top + 74.0;
            if screen_x >= row_left
                && screen_x <= row_right
                && screen_y >= row1_top
                && screen_y <= row1_top + 58.0
            {
                hovered_setting = Some(0);
            } else if screen_x >= row_left
                && screen_x <= row_right
                && screen_y >= row2_top
                && screen_y <= row2_top + 58.0
            {
                hovered_setting = Some(1);
            }

            let distance_y = top + 184.0;
            let minus_x = right - 112.0;
            let plus_x = right - 58.0;
            if screen_y >= distance_y
                && screen_y <= distance_y + 42.0
                && screen_x >= minus_x
                && screen_x <= minus_x + 42.0
            {
                hovered_distance = Some(0);
            } else if screen_y >= distance_y
                && screen_y <= distance_y + 42.0
                && screen_x >= plus_x
                && screen_x <= plus_x + 42.0
            {
                hovered_distance = Some(1);
            }
        }
    }
    crate::game::chetars_spatial_gui::set_hovered_setting(hovered_setting);
    crate::game::chetars_spatial_gui::set_hovered_distance_control(hovered_distance);

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

    let switch_progress = if crate::game::chetars_spatial_gui::selected_button() == 2 {
        Some(crate::game::chetars_spatial_gui::advance_switch_animations())
    } else {
        None
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

        label(game, draw, panel.body[0][0] + 28.0, panel.body[0][1] + 22.0, "CHETARS", 0xFFFFFFFF);
        label(
            game,
            draw,
            panel.body[0][0] + 110.0,
            panel.body[0][1] + 24.0,
            "SPATIAL DISPLAY",
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
                // The two mutually exclusive placement switches are the
                // actual settings requested: keep a world anchor or follow.
                let [pin_progress, follow_progress] = switch_progress.unwrap_or([1.0, 0.0]);
                let row_height = 58.0;
                let row1_y = top + 30.0;
                let row2_y = row1_y + 74.0;
                let selected = crate::game::chetars_spatial_gui::selected_setting();

                label(game, draw, row_x, top + 12.0, "PANEL BEHAVIOR", 0xFF303844);

                let draw_switch = |game: &crate::game::Game<'_>,
                                   draw: &mut NativeWorldDraw,
                                   y: f32,
                                   progress: f32,
                                   is_selected: bool| {
                    let row_color = if is_selected { 0xFFEAF5FF } else { 0xFFF2F4F7 };
                    paint(game, draw, rect(row_x, y, row_width, row_height, 9.0), row_color);
                    if is_selected {
                        paint(game, draw, rect(row_x, y, 3.0, row_height, 10.0), 0xFF33AAFF);
                    }
                    let track_w = 54.0;
                    let track_h = 24.0;
                    let track_x = row_x + row_width - track_w - 14.0;
                    let track_y = y + (row_height - track_h) * 0.5;
                    let blend = progress.clamp(0.0, 1.0);
                    let red = (0x65 as f32 + (0x33 as f32 - 0x65 as f32) * blend) as u32;
                    let green = (0x6B as f32 + (0xAA as f32 - 0x6B as f32) * blend) as u32;
                    let blue = (0x78 as f32 + (0xFF as f32 - 0x78 as f32) * blend) as u32;
                    let track_color = 0xFF000000 | (red << 16) | (green << 8) | blue;
                    paint(game, draw, rect(track_x, track_y, track_w, track_h, 11.0), track_color);
                    let knob_size = 18.0;
                    let knob_x = track_x + 3.0 + blend * (track_w - knob_size - 6.0);
                    paint(
                        game,
                        draw,
                        rect(knob_x, track_y + 3.0, knob_size, knob_size, 12.0),
                        0xFFFFFFFF,
                    );
                };

                draw_switch(game, draw, row1_y, pin_progress, selected == 0);
                draw_switch(game, draw, row2_y, follow_progress, selected == 1);
                label(game, draw, row_x + 12.0, row1_y + 17.0, "KEEP AT COORDINATES", 0xFF202833);
                label(game, draw, row_x + 12.0, row1_y + 36.0, "Stays at the saved world position", 0xFF687482);
                label(game, draw, row_x + 12.0, row2_y + 17.0, "FOLLOW ME", 0xFF202833);
                label(game, draw, row_x + 12.0, row2_y + 36.0, "Follows your position and keeps its angle", 0xFF687482);

                let distance_y = top + 184.0;
                paint(game, draw, rect(row_x, distance_y, row_width, 42.0, 9.0), 0xFFF2F4F7);
                label(game, draw, row_x + 12.0, distance_y + 25.0, "DISTANCE", 0xFF202833);
                let value_x = row_x + row_width - 184.0;
                label(game, draw, value_x, distance_y + 25.0, distance_text.as_str(), 0xFF303844);
                let minus_x = right - 112.0;
                let plus_x = right - 58.0;
                paint(game, draw, rect(minus_x, distance_y + 5.0, 42.0, 32.0, 11.0),
                    if crate::game::chetars_spatial_gui::hovered_distance_control() == Some(0) { 0xFF33AAFF } else { 0xFF4B5563 });
                paint(game, draw, rect(plus_x, distance_y + 5.0, 42.0, 32.0, 11.0),
                    if crate::game::chetars_spatial_gui::hovered_distance_control() == Some(1) { 0xFF33AAFF } else { 0xFF4B5563 });
                label(game, draw, minus_x + 15.0, distance_y + 25.0, "-", 0xFFFFFFFF);
                label(game, draw, plus_x + 14.0, distance_y + 25.0, "+", 0xFFFFFFFF);
                label(game, draw, row_x, distance_y + 58.0, "Mouse wheel on the top bar also changes distance", 0xFF687482);
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
    if follow {
        let (panel_yaw, panel_pitch, panel_roll) =
            panel_rotation.to_euler(glam::EulerRot::YXZ);
        draw.rotate_about([0.0, 1.0, 0.0], panel_yaw.to_degrees(), [0.0; 3]);
        draw.rotate_about([1.0, 0.0, 0.0], panel_pitch.to_degrees(), [0.0; 3]);
        draw.rotate_about([0.0, 0.0, 1.0], panel_roll.to_degrees(), [0.0; 3]);
        let (orbit_yaw, orbit_pitch) = crate::game::chetars_spatial_gui::orbit_angles();
        let local_offset = crate::game::chetars_spatial_gui::orbit_offset(
            orbit_yaw,
            orbit_pitch,
            distance,
        );
        draw.translate(local_offset.to_array());
    } else {
        let (panel_yaw, panel_pitch, panel_roll) =
            panel_rotation.to_euler(glam::EulerRot::YXZ);
        let _ = draw.translate_world(panel_position.to_array());
        draw.rotate_about([0.0, 1.0, 0.0], panel_yaw.to_degrees(), [0.0; 3]);
        draw.rotate_about([1.0, 0.0, 0.0], panel_pitch.to_degrees(), [0.0; 3]);
        draw.rotate_about([0.0, 0.0, 1.0], panel_roll.to_degrees(), [0.0; 3]);
    }
    draw.scale([world_per_pixel, world_per_pixel, world_per_pixel]);

    draw_face(game, draw);

    // Render a second, readable UI on the reverse side.
    draw.push();
    draw.rotate_about([0.0, 1.0, 0.0], 180.0, [0.0; 3]);
    draw_face(game, draw);
    draw.pop();

    draw.pop();
}
