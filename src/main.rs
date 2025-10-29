mod gui;
mod instances;
mod settings;

#[tokio::main]
async fn main() {
    let settings = settings::get();
    let instances = instances::get();
    if let Some(error) = gui::start(settings, instances).err() {
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_inner_size([160.0, 64.0]),
            ..Default::default()
        };

        let _ = eframe::run_simple_native("Error", options, move |ctx, _frame| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.label("Chorus has just crashed!");
                ui.label(format!("Error: {}", error.to_string()));
                if ui.button("Exit").clicked() {
                    return;
                }
            });
        });
    }
    println!("Hello, world!");
}
