use egui::{RichText, Ui};

use crate::interface::{components::panel::Panel, theme::AnimrusTheme};

pub struct LayerPanel;

impl LayerPanel {
    pub fn show(ui: &mut Ui) {
        egui::ScrollArea::both()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                Panel::frame().show(ui, |ui| {
                    ui.vertical(|ui| {
                        Self::header(ui);

                        Panel::separator(ui);

                        Self::layer_item(ui, "Sketch", true);
                        Self::layer_item(ui, "Color", false);
                        Self::layer_item(ui, "Background", false);

                        ui.add_space(8.0);

                        Panel::secondary_text(ui, "3 layers");
                    });
                });
            });
    }

    fn header(ui: &mut Ui) {
        ui.horizontal(|ui| {
            Panel::heading(ui, "Layers");

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.small_button("⋯").clicked() {}
                if ui.small_button("+").clicked() {}
            });
        });
    }

    fn layer_item(ui: &mut Ui, name: &str, selected: bool) {
        let background = if selected {
            AnimrusTheme::SELECTION
        } else {
            AnimrusTheme::SURFACE
        };

        egui::Frame::new()
            .fill(background)
            .inner_margin(egui::Margin::same(8))
            .corner_radius(4)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("⠿").color(AnimrusTheme::TEXT_SECONDARY));

                    ui.label(RichText::new("◉").color(AnimrusTheme::TEXT_SECONDARY));

                    ui.label(RichText::new(name).color(AnimrusTheme::TEXT_PRIMARY));
                });
            });

        ui.add_space(3.0);
    }
}
