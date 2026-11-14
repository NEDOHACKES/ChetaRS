fn render(
    game: &crate::game::Game<'_>,
    batch: &mut HudBatch,
    screen: &NativeScreenView<'_>,
) {
    if screen.id != "mod"
        || game.native_mod_screen() != Some("chetars_spatial_gui")
    {
        return;
    }

    let panel = crate::game::chetars_spatial_gui::panel(screen.mouse, batch.width, batch.height);
    let start = batch.vertices.len();

    // HudAtlas::white is crate-private in Horizon 0.2.65.
    // Use the public atlas sprite map instead.
    let white = game
        .gui
        .atlas
        .sprites
        .get("horizon/white")
        .copied()
        .unwrap_or_else(|| {
            game.gui
                .atlas
                .sprites
                .get("minecraft:white")
                .copied()
                .expect("Horizon white atlas sprite is missing")
        });
    let uv = [
        [(white.u0 + white.u1) * 0.5, (white.v0 + white.v1) * 0.5];
        4
    ];

    batch.quad_3d(panel.shadow, uv, 0xB0000000);
    batch.quad_3d(panel.body, uv, 0xE9101720);
    batch.quad_3d(panel.inner, uv, 0xF0192430);
    batch.quad_3d(panel.header, uv, 0xF02A3444);

    for i in 0..3 {
        let color = if crate::game::chetars_spatial_gui::selected_button() == i {
            0xF05A86B8
        } else if crate::game::chetars_spatial_gui::button_hit(
            screen.mouse,
            batch.width,
            batch.height,
            i,
        ) {
            0xF046536A
        } else {
            0xD82E3748
        };
        batch.quad_3d(panel.buttons[i], uv, color);
    }

    batch.quad_3d(panel.accent_left, uv, 0xFF6EA8FF);
    batch.quad_3d(panel.accent_top, uv, 0xFF9BC7FF);

    batch.depth_since(start, [0.0, 0.0, batch.width, batch.height]);

    let title_x = panel.body[0][0] + 28.0;
    let title_y = panel.body[0][1] + 24.0;
    batch.text(&game.gui.atlas, "CHETARS", title_x, title_y, 0xFFFFFFFF, true);
    batch.text(
        &game.gui.atlas,
        "SPATIAL MENU",
        title_x,
        title_y + 16.0,
        0xFF9BC7FF,
        false,
    );

    for (i, label) in ["HOME", "PROFILE", "SETTINGS"].iter().enumerate() {
        let x = panel.buttons[i][0][0] + 18.0;
        let y = panel.buttons[i][0][1] + 10.0;
        let color = if crate::game::chetars_spatial_gui::selected_button() == i {
            0xFFFFFFFF
        } else {
            0xFFD5DCE8
        };
        batch.text(&game.gui.atlas, label, x, y, color, true);
    }

    batch.text(
        &game.gui.atlas,
        "F6  OPEN/CLOSE",
        panel.body[3][0] + 26.0,
        panel.body[3][1] - 22.0,
        0xFF93A4BA,
        false,
    );
}
