use egui::{Color32, RichText, Stroke, Ui, Vec2};

use crate::interface::theme::AnimrusTheme;

pub struct Menu;

impl Menu {
    pub fn show(ui: &mut Ui) {
        ui.centered_and_justified(|ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                let style = ui.style_mut();

                // Give menu items comfortable horizontal & vertical breathing room.
                style.spacing.item_spacing = Vec2::new(10.0, 4.0);
                style.spacing.button_padding = Vec2::new(10.0, 6.0);

                // Flat top-level menu items; hover & open states stay clear.
                style.visuals.widgets.inactive.bg_fill = Color32::TRANSPARENT;
                style.visuals.widgets.inactive.weak_bg_fill = Color32::TRANSPARENT;
                style.visuals.widgets.inactive.bg_stroke = Stroke::NONE;
                style.visuals.widgets.hovered.bg_fill = AnimrusTheme::SURFACE;
                style.visuals.widgets.hovered.weak_bg_fill = AnimrusTheme::SURFACE;
                style.visuals.widgets.hovered.corner_radius = 4.into();
                style.visuals.widgets.open.bg_fill = AnimrusTheme::SELECTION;
                style.visuals.widgets.open.weak_bg_fill = AnimrusTheme::SELECTION;
                style.visuals.widgets.open.corner_radius = 4.into();

                Self::menu_button(ui, "File", |ui| {
                    if ui.button("New Project").clicked() {
                        ui.close();
                    }
                    if ui.button("Open Project").clicked() {
                        ui.close();
                    }
                    ui.separator();
                    if ui.button("Save").clicked() {
                        ui.close();
                    }
                    if ui.button("Save As").clicked() {
                        ui.close();
                    }
                });

                Self::menu_button(ui, "Edit", |ui| {
                    if ui.button("Undo").clicked() {
                        ui.close();
                    }
                    if ui.button("Redo").clicked() {
                        ui.close();
                    }
                    ui.separator();
                    if ui.button("Preferences").clicked() {
                        ui.close();
                    }
                });

                Self::menu_button(ui, "View", |ui| {
                    if ui.button("Zoom In").clicked() {
                        ui.close();
                    }
                    if ui.button("Zoom Out").clicked() {
                        ui.close();
                    }
                    if ui.button("Reset View").clicked() {
                        ui.close();
                    }
                });

                Self::menu_button(ui, "Animation", |ui| {
                    if ui.button("Play").clicked() {
                        ui.close();
                    }
                    if ui.button("Pause").clicked() {
                        ui.close();
                    }
                    ui.separator();
                    if ui.button("Previous Frame").clicked() {
                        ui.close();
                    }
                    if ui.button("Next Frame").clicked() {
                        ui.close();
                    }
                });

                Self::menu_button(ui, "Help", |ui| {
                    if ui.button("Documentation").clicked() {
                        ui.close();
                    }
                    if ui.button("About Animrus").clicked() {
                        ui.close();
                    }
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
