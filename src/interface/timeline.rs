use egui::{RichText, Ui};

use super::{panel::Panel, theme::AnimrusTheme};

pub struct Timeline;

impl Timeline {
    pub fn show(ui: &mut Ui) {
        Panel::frame().show(ui, |ui| {
            ui.vertical(|ui| {
                Self::toolbar(ui);

                Panel::separator(ui);

                Self::ruler(ui);

                Panel::separator(ui);

                Self::tracks(ui);
            });
        });
    }

    fn toolbar(ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.button("⏮");
            ui.button("▶");
            ui.button("⏭");

            ui.separator();

            ui.label(RichText::new("24 FPS").color(AnimrusTheme::TEXT_SECONDARY));

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(RichText::new("Frame 1").color(AnimrusTheme::TEXT_SECONDARY));
            });
        });
    }

    fn ruler(ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.add_sized(
                [100.0, 22.0],
                egui::Label::new(RichText::new("Frames").color(AnimrusTheme::TEXT_SECONDARY)),
            );

            for frame in (1..=50).step_by(5) {
                ui.add_sized(
                    [35.0, 22.0],
                    egui::Label::new(
                        RichText::new(frame.to_string())
                            .size(11.0)
                            .color(AnimrusTheme::TEXT_SECONDARY),
                    ),
                );
            }
        });
    }

    fn tracks(ui: &mut Ui) {
        for name in ["Sketch", "Color", "Background"] {
            ui.horizontal(|ui| {
                ui.add_sized(
                    [100.0, 24.0],
                    egui::Label::new(RichText::new(name).color(AnimrusTheme::TEXT_PRIMARY)),
                );

                ui.allocate_ui(egui::vec2(35.0, 18.0), |ui| {
                    egui::Frame::new()
                        .fill(AnimrusTheme::SELECTION)
                        .corner_radius(3)
                        .show(ui, |_| {});
                });
            });
        }
    }
}
