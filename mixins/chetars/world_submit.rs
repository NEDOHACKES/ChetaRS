fn world_submit(
    game: &crate::game::Game<'_>,
    draw: &mut NativeWorldDraw,
) {
    if game.native_mod_screen() != Some("chetars_spatial_gui") {
        return;
    }

    // Do not call game.native_camera() here: source-mixin mods are not
    // necessarily registered as mod_api::Hooks, so that accessor may be None.
    // NativeWorldDraw already starts in the current frame's camera-relative
    // world space, and camera_orientation() gives the view rotation.
    let (width, height, mouse) = crate::game::chetars_spatial_gui::viewport();
    if width <= 0.0 || height <= 0.0 {
        return;
    }

    let panel = crate::game::chetars_spatial_gui::panel(mouse, width, height);
    let centre_x = width * 0.5;
    let centre_y = height * 0.5;

    // Place the display in front of the camera, as a Spatial-GUI-style
    // floating screen. Its size follows the current vertical FOV.
    let distance = 2.25f32;
    // Keep the display a consistent world-space width across window sizes.
    // The vanilla FOV is intentionally not required for this calculation.
    let world_per_pixel = (3.6 / panel.body[1][0].abs().max(width * 0.5))
        .clamp(0.001, 0.02);

    let (yaw, pitch, roll) =
        game.camera_orientation().to_euler(glam::EulerRot::YXZ);

    draw.push();
    draw.rotate_about([0.0, 1.0, 0.0], yaw.to_degrees(), [0.0; 3]);
    draw.rotate_about([1.0, 0.0, 0.0], pitch.to_degrees(), [0.0; 3]);
    draw.rotate_about([0.0, 0.0, 1.0], roll.to_degrees(), [0.0; 3]);
    draw.translate([0.0, 0.0, -distance]);
    draw.scale([world_per_pixel, world_per_pixel, world_per_pixel]);

    let paint = |game: &crate::game::Game<'_>,
                 draw: &mut NativeWorldDraw,
                 points: [[f32; 3]; 4],
                 argb: u32| {
        // The display is authored as screen-space TL, TR, BR, BL quads.
        // NativeWorldDraw receives the winding that faces the camera.
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

        // Horizon's public white atlas sprite is used for solid colour
        // surfaces; no inventory or container texture is referenced.
        let _ = game.native_world_quad(
            draw,
            "horizon/white",
            corners,
            [0.0, 0.0, 16.0, 16.0],
            rgba,
        );
    };

    // Floating body and its offset shadow create the spatial screen frame.
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

    // The only content surface: a blank, opaque white area.
    // No player inventory is opened, copied, rendered or modified.
    paint(game, draw, panel.content_frame, 0xFF4A4A4A);
    paint(game, draw, panel.white_area, 0xFFFFFFFF);

    for i in 0..3 {
        let color = if crate::game::chetars_spatial_gui::selected_button() == i {
            0xFF33AAFF
        } else if crate::game::chetars_spatial_gui::button_hit(
            mouse,
            width,
            height,
            i,
        ) {
            0xFF4A4A4A
        } else {
            0xFF2C2C2C
        };
        paint(game, draw, panel.buttons[i], color);
    }

    paint(game, draw, panel.accent_left, 0xFF33AAFF);
    paint(game, draw, panel.accent_top, 0xFF33AAFF);

    let label = |game: &crate::game::Game<'_>,
                 draw: &mut NativeWorldDraw,
                 x: f32,
                 y: f32,
                 text: &str,
                 color: u32| {
        draw.push();
        // Text glyphs use downward GUI Y; the panel's local Y points upward.
        draw.translate([x - centre_x, centre_y - y, 14.0]);
        draw.scale([1.0, -1.0, 1.0]);
        let text = crate::text::Text::literal(text);
        let _ = game.native_world_text(draw, &text, color, None);
        draw.pop();
    };

    let title_x = panel.body[0][0] + 28.0;
    let title_y = panel.body[0][1] + 22.0;
    label(game, draw, title_x, title_y, "CHETARS", 0xFFFFFFFF);
    label(
        game,
        draw,
        title_x + 82.0,
        title_y + 2.0,
        "SPATIAL DISPLAY",
        0xFFCCCCCC,
    );

    for (i, text) in ["HOME", "PROFILE", "SETTINGS"].iter().enumerate() {
        let x = panel.buttons[i][0][0] + 14.0;
        let y = panel.buttons[i][0][1] + 12.0;
        let color = if crate::game::chetars_spatial_gui::selected_button() == i {
            0xFFFFFFFF
        } else {
            0xFFCCCCCC
        };
        label(game, draw, x, y, text, color);
    }

    label(
        game,
        draw,
        panel.body[3][0] + 22.0,
        panel.body[3][1] - 22.0,
        "F6  CLOSE",
        0xFFCCCCCC,
    );

    draw.pop();
}
