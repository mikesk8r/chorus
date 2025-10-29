use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use crate::{
    instances::{Instance, InstanceGroup},
    settings::Settings,
};

mod settings;
use settings::*;

pub fn start(_settings: Settings, instances: InstanceGroup) -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([640.0, 480.0])
            .with_icon(Arc::new(egui::IconData {
                rgba: image::load_from_memory(include_bytes!("../../assets/icon.png"))
                    .unwrap()
                    .to_rgba8()
                    .to_vec(),
                width: 128,
                height: 128,
            })),
        ..Default::default()
    };

    let mut settings_shown = false;
    let mut about_shown = false;

    let selected_instance: Rc<RefCell<Instance>> = Rc::new(RefCell::new(Instance::default()));

    let mut settings_menu_state = SettingsMenus::General;

    // TODO: dynamically allocate space for Modals' closing buttons
    eframe::run_simple_native("Chorus", options, move |ctx, _frame| {
        egui::TopBottomPanel::top("topbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Settings").clicked() {
                    settings_shown = true;
                }
                if ui.button("About").clicked() {
                    about_shown = true;
                }
            });
        });
        egui::SidePanel::left("instance_list").show(ctx, |ui| {
            instances.draw_recursive(ui, Rc::clone(&selected_instance));
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            let selected = selected_instance.borrow();
            ui.heading(selected.name.clone());
        });
        if settings_shown {
            (settings_shown, settings_menu_state) =
                show_settings(ctx, settings_shown, settings_menu_state);
        }
        if about_shown {
            egui::Modal::new("about".into()).show(ctx, |ui| {
                ui.label(format!("Chorus version {}", env!("CARGO_PKG_VERSION")));
                ui.allocate_space(egui::Vec2 { x: 200.0, y: 0.0 });
                if ui.button("Close").clicked() {
                    about_shown = false;
                }
            });
        }
    })
}
