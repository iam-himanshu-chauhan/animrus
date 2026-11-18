pub mod document;

use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Animrus")
            .with_inner_size([1280.0, 800.0])
            .with_min_inner_size([800.0, 600.0])
            .with_maximized(true),
        ..Default::default()
    };


    eframe::run_native(
        "Animrus",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::dark());

            Ok(Box::new(AnimrusApp::default()))
        }),
    )
}

#[derive(Default)]
struct AnimrusApp {
    // Application state will live here.
}

impl eframe::App for AnimrusApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(180.0);

                ui.heading("ANIMRUS");
                ui.label("Raster Animation Studio");

                ui.add_space(24.0);

                if ui.button("Create New Project").clicked() {
                    println!("Create project clicked");
                }
            });
        });
    }
}
