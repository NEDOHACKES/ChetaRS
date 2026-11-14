fn world_submit(
    game: &crate::game::Game<'_>,
    draw: &mut NativeWorldDraw,
) {
    if game.native_mod_screen() != Some("chetars_spatial_gui") {
        return;
    }

    let Some(camera) = game.native_camera() else {
        return;
    };

    let (width, height, mouse) = crate::game::chetars_spatial_gui::viewport();
    if width <= 0.0 || height <= 0.0 {
        return;
    }

    let panel = crate::game::chetars_spatial_gui::panel(mouse, width, height);
    let centre_x = width * 0.5;
    let centre_y = height * 0.5;

    // Fit a virtual GUI pixel to the current vertical FOV. The canvas is
    // 2.25 blocks in front of the camera and remains the same screen fraction
    // at different resolutions and FOV settings.
    let distance = 2.25f32;
    let fov = camera.fov_degrees.to_radians().clamp(20.0f32.to_radians(), 150.0f32.to_radians());
    let world_per_pixel =
        (2.0 * distance * (fov * 0.5).tan() / height).clamp(0.0005, 0.05);

    // Game::camera_orientation is the same camera rotation used by the world
    // renderer. EulerRot::YXZ preserves Horizon's yaw-then-pitch convention.
    let (yaw, pitch, roll) = game.camera_orientation().to_euler(glam::EulerRot::YXZ);

    draw.push();
    draw.rotate_about([0.0, 1.0, 0.0], yaw.to_degrees(), [0.0; 3]);
    draw.rotate_about([1.0, 0.0, 0.0], pitch.to_degrees(), [0.0; 3]);
    draw.rotate_about([0.0, 0.0, 1.0], roll.to_degrees(), [0.0; 3]);
    draw.translate([0.0, 0.0, -distance]);
    // The canvas lives in camera space, but all quads are submitted as
    // camera-relative world geometry and receive real world depth testing.
    draw.scale([world_per_pixel, world_per_pixel, 0.00035]);

    let paint = |game: &crate::game::Game<'_>,
                 draw: &mut NativeWorldDraw,
                 points: [[f32; 3]; 4],
                 argb: u32| {
        // Runtime panel points are TL, TR, BR, BL. Native world quads use
        // TL, BL, BR, TR so their front face points back toward the camera.
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

        if !game.native_world_quad(
            draw,
            "horizon/white",
            corners,
            [0.0, 0.0, 16.0, 16.0],
            rgba,
        ) {
            let _ = game.native_world_quad(
                draw,
                "minecraft:white",
                corners,
                [0.0, 0.0, 16.0, 16.0],
                rgba,
            );
        }
    };

    paint(game, draw, panel.shadow, 0xB0000000);
    paint(game, draw, panel.body, 0xE9101720);
    paint(game, draw, panel.inner, 0xF0192430);
    paint(game, draw, panel.header, 0xF02A3444);

    for i in 0..3 {
        let color = if crate::game::chetars_spatial_gui::selected_button() == i {
            0xF05A86B8
        } else if crate::game::chetars_spatial_gui::button_hit(
            mouse,
            width,
            height,
            i,
        ) {
            0xF046536A
        } else {
            0xD82E3748
        };
        paint(game, draw, panel.buttons[i], color);
    }

    paint(game, draw, panel.accent_left, 0xFF6EA8FF);
    paint(game, draw, panel.accent_top, 0xFF9BC7FF);

    let label = |game: &crate::game::Game<'_>,
                 draw: &mut NativeWorldDraw,
                 x: f32,
                 y: f32,
                 text: &str,
                 color: u32| {
        draw.push();
        // HUD font coordinates grow downward; flip only the text's local Y.
        draw.scale([1.0, -1.0, 1.0]);
        draw.translate([x - centre_x, y - centre_y, 14.0]);
        let text = crate::text::Text::literal(text);
        let _ = game.native_world_text(draw, &text, color, None);
        draw.pop();
    };

    let title_x = panel.body[0][0] + 28.0;
    let title_y = panel.body[0][1] + 24.0;
    label(game, draw, title_x, title_y, "CHETARS", 0xFFFFFFFF);
    label(
        game,
        draw,
        title_x,
        title_y + 16.0,
        "SPATIAL MENU",
        0xFF9BC7FF,
    );

    for (i, text) in ["HOME", "PROFILE", "SETTINGS"].iter().enumerate() {
        let x = panel.buttons[i][0][0] + 18.0;
        let y = panel.buttons[i][0][1] + 10.0;
        let color = if crate::game::chetars_spatial_gui::selected_button() == i {
            0xFFFFFFFF
        } else {
            0xFFD5DCE8
        };
        label(game, draw, x, y, text, color);
    }

    label(
        game,
        draw,
        panel.body[3][0] + 26.0,
        panel.body[3][1] - 22.0,
        "F6  OPEN/CLOSE",
        0xFF93A4BA,
    );

    draw.pop();
}
