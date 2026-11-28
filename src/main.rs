pub mod cache_sys;
pub mod core;
pub mod dochandler;
pub mod interface;
pub mod rendering;

use core::app::AnimrusApp;
use eframe::egui;

use crate::interface::theme::AnimrusTheme;

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

            AnimrusTheme::apply(&cc.egui_ctx);
            Ok(Box::new(AnimrusApp::new("Learning".into())))
        }),
    )
}
