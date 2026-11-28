use egui::{RichText, Ui};

use super::theme::AnimrusTheme;

pub struct Menu;

impl Menu {
    pub fn show(ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 12.0;

            Self::brand(ui);

            ui.add_space(12.0);

            ui.separator();

            Self::menu_button(ui, "File", |ui| {
                ui.button("New Project");
                ui.button("Open Project");
                ui.separator();
                ui.button("Save");
                ui.button("Save As");
            });

            Self::menu_button(ui, "Edit", |ui| {
                ui.button("Undo");
                ui.button("Redo");
                ui.separator();
                ui.button("Preferences");
            });

            Self::menu_button(ui, "View", |ui| {
                ui.button("Zoom In");
                ui.button("Zoom Out");
                ui.button("Reset View");
            });

            Self::menu_button(ui, "Animation", |ui| {
                ui.button("Play");
                ui.button("Pause");
                ui.separator();
                ui.button("Previous Frame");
                ui.button("Next Frame");
            });

            Self::menu_button(ui, "Help", |ui| {
                ui.button("Documentation");
                ui.button("About Animrus");
            });

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let settings = ui.button(
                    RichText::new("⚙")
                        .size(17.0)
                        .color(AnimrusTheme::TEXT_SECONDARY),
                );

                settings.on_hover_text("Settings");
            });
        });
    }

    fn brand(ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(
                RichText::new("A")
                    .size(18.0)
                    .strong()
                    .color(AnimrusTheme::ACCENT),
            );

            ui.label(
                RichText::new("Animrus")
                    .size(14.0)
                    .strong()
                    .color(AnimrusTheme::TEXT_PRIMARY),
            );
        });
    }

    fn menu_button(ui: &mut Ui, title: &str, content: impl FnOnce(&mut Ui)) {
        ui.menu_button(
            RichText::new(title)
                .size(13.0)
                .color(AnimrusTheme::TEXT_PRIMARY),
            content,
        );
    }
}
