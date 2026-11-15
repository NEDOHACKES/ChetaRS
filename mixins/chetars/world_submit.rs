fn world_submit(
    game: &crate::game::Game<'_>,
    draw: &mut NativeWorldDraw,
) {
    if !game.native_playing() || !crate::game::chetars_spatial_gui::is_open() {
        return;
    }

    let (width, height, mouse) = crate::game::chetars_spatial_gui::viewport();
    if width <= 0.0 || height <= 0.0 {
        return;
    }

    let panel = crate::game::chetars_spatial_gui::panel(mouse, width, height);
    let centre_x = width * 0.5;
    let centre_y = height * 0.5;
    let distance = 2.25f32;
    let current_rotation = game.camera_orientation();
    let (yaw, pitch, _) = current_rotation.to_euler(glam::EulerRot::YXZ);

    crate::game::chetars_spatial_gui::update_drag_view(yaw, pitch, distance);

    let follow = crate::game::chetars_spatial_gui::follow_mode();
    let camera_position = glam::DVec3::from_array(draw.camera);

    if !follow && !crate::game::chetars_spatial_gui::anchor_ready() {
        let forward = current_rotation * glam::Vec3::NEG_Z;
        let anchor_position = camera_position + forward.as_dvec3() * distance as f64;
        crate::game::chetars_spatial_gui::save_anchor(
            anchor_position.to_array(),
            current_rotation,
        );
    }

    let (panel_position, panel_rotation) = if follow {
        let offset = crate::game::chetars_spatial_gui::follow_offset();
        let local = glam::Vec3::new(offset[0], offset[1], -distance);
        (
            camera_position + (current_rotation * local).as_dvec3(),
            current_rotation,
        )
    } else {
        let (position, rotation) = crate::game::chetars_spatial_gui::anchor();
        (glam::DVec3::from_array(position), rotation)
    };

    // The panel follows the camera only in Follow mode. In pinned mode its
    // world position and orientation remain unchanged as the player looks around.
    let panel_width_pixels = (panel.body[1][0] - panel.body[0][0]).abs().max(1.0);
    let world_per_pixel = (3.6 / panel_width_pixels).clamp(0.001, 0.02);

    draw.push();
    if follow {
        let (panel_yaw, panel_pitch, panel_roll) =
            panel_rotation.to_euler(glam::EulerRot::YXZ);
        draw.rotate_about([0.0, 1.0, 0.0], panel_yaw.to_degrees(), [0.0; 3]);
        draw.rotate_about([1.0, 0.0, 0.0], panel_pitch.to_degrees(), [0.0; 3]);
        draw.rotate_about([0.0, 0.0, 1.0], panel_roll.to_degrees(), [0.0; 3]);
        let offset = crate::game::chetars_spatial_gui::follow_offset();
        draw.translate([offset[0], offset[1], -distance]);
    } else {
        let (panel_yaw, panel_pitch, panel_roll) =
            panel_rotation.to_euler(glam::EulerRot::YXZ);
        let _ = draw.translate_world(panel_position.to_array());
        draw.rotate_about([0.0, 1.0, 0.0], panel_yaw.to_degrees(), [0.0; 3]);
        draw.rotate_about([1.0, 0.0, 0.0], panel_pitch.to_degrees(), [0.0; 3]);
        draw.rotate_about([0.0, 0.0, 1.0], panel_roll.to_degrees(), [0.0; 3]);
    }
    draw.scale([world_per_pixel, world_per_pixel, world_per_pixel]);

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

    // Floating frame, side walls, header, rail and the blank content canvas.
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
            } else {
                0xFFCCCCCC
            },
        );
    }

    // SETTINGS: two mutually exclusive animated switches.
    if crate::game::chetars_spatial_gui::selected_button() == 2 {
        let [pin_progress, follow_progress] =
            crate::game::chetars_spatial_gui::advance_switch_animations();
        let left = panel.white_area[0][0];
        let top = panel.white_area[0][1];
        let right = panel.white_area[1][0];
        let row_x = left + 22.0;
        let row_width = (right - left - 44.0).max(120.0);
        let row_height = 62.0;
        let row1_y = top + 32.0;
        let row2_y = row1_y + 78.0;
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
            let knob_y = track_y + 3.0;
            paint(game, draw, rect(knob_x, knob_y, knob_size, knob_size, 12.0), 0xFFFFFFFF);
        };

        draw_switch(game, draw, row1_y, pin_progress, selected == 0);
        draw_switch(game, draw, row2_y, follow_progress, selected == 1);

        label(game, draw, row_x + 12.0, row1_y + 18.0, "KEEP AT COORDINATES", 0xFF202833);
        label(game, draw, row_x + 12.0, row1_y + 36.0, "Stays fixed while you look around", 0xFF687482);
        label(game, draw, row_x + 12.0, row2_y + 18.0, "FOLLOW ME", 0xFF202833);
        label(game, draw, row_x + 12.0, row2_y + 36.0, "Moves with your camera", 0xFF687482);
    }

    label(
        game,
        draw,
        panel.body[3][0] + 22.0,
        panel.body[3][1] - 22.0,
        "RMB HOLD + LOOK TO MOVE  |  ARROWS + ENTER IN SETTINGS",
        0xFFCCCCCC,
    );

    // Project the center crosshair ray onto the panel. A left click on one of
    // the settings rows is handled by the raw mouse-button hook.
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
            Some([
                centre_x + local_hit.x / world_per_pixel,
                centre_y - local_hit.y / world_per_pixel,
            ])
        } else {
            None
        }
    } else {
        None
    };

    let mut hover = None;
    if crate::game::chetars_spatial_gui::selected_button() == 2 {
        if let Some([screen_x, screen_y]) = hit {
            let left = panel.white_area[0][0];
            let top = panel.white_area[0][1];
            let right = panel.white_area[1][0];
            let row_left = left + 22.0;
            let row_right = right - 22.0;
            let row1_top = top + 32.0;
            let row2_top = row1_top + 78.0;
            if screen_x >= row_left
                && screen_x <= row_right
                && screen_y >= row1_top
                && screen_y <= row1_top + 62.0
            {
                hover = Some(0);
            } else if screen_x >= row_left
                && screen_x <= row_right
                && screen_y >= row2_top
                && screen_y <= row2_top + 62.0
            {
                hover = Some(1);
            }
        }
    }
    crate::game::chetars_spatial_gui::set_hovered_setting(hover);

    draw.pop();
}
