fn mouse(screen: &NativeScreenView<'_>, button: i32, canceled: &mut bool) {
    if button != 0 || screen.id != "mod" {
        return;
    }

    if crate::game::chetars_spatial_gui::button_hit(screen.mouse, 0)
        || crate::game::chetars_spatial_gui::button_hit(screen.mouse, 1)
        || crate::game::chetars_spatial_gui::button_hit(screen.mouse, 2)
    {
        crate::game::chetars_spatial_gui::select_button(
            crate::game::chetars_spatial_gui::button_at(screen.mouse),
        );
        *canceled = true;
    }
}
