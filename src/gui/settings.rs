#[derive(Copy, Clone, PartialEq)]
pub enum SettingsMenus {
    General = 0,
    Java = 1,
}

pub fn show_settings(
    ctx: &egui::Context,
    mut settings_shown: bool,
    mut settings_menu_state: SettingsMenus,
) -> (bool, SettingsMenus) {
    egui::Modal::new("settings".into()).show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.selectable_value(&mut settings_menu_state, SettingsMenus::General, "General");
            ui.selectable_value(&mut settings_menu_state, SettingsMenus::Java, "Java");
        });

        ui.horizontal(|ui| {
            ui.allocate_space(egui::Vec2 { x: 200.0, y: 0.0 });
            if ui.button("Close").clicked() {
                settings_shown = false;
            }
        });
    });
    (settings_shown, settings_menu_state)
}
