use egui::{Frame, Margin, Stroke, Ui};

use super::theme::AnimrusTheme;

pub struct Panel;

impl Panel {
    /// Creates a standard Animrus panel frame.
    pub fn frame() -> Frame {
        Frame::new()
            .fill(AnimrusTheme::PANEL)
            .stroke(Stroke::new(1.0, AnimrusTheme::BORDER))
            .inner_margin(Margin::same(AnimrusTheme::PANEL_PADDING as i8))
    }

    /// Draws a panel heading.
    pub fn heading(ui: &mut Ui, title: &str) {
        ui.label(
            egui::RichText::new(title)
                .size(13.0)
                .strong()
                .color(AnimrusTheme::TEXT_PRIMARY),
        );
    }

    /// Draws a subtle separator.
    pub fn separator(ui: &mut Ui) {
        ui.add_space(4.0);

        ui.painter().hline(
            ui.min_rect().left()..=ui.max_rect().right(),
            ui.cursor().top(),
            Stroke::new(1.0, AnimrusTheme::BORDER),
        );

        ui.add_space(4.0);
    }

    /// Draws secondary panel text.
    pub fn secondary_text(ui: &mut Ui, text: &str) {
        ui.label(
            egui::RichText::new(text)
                .size(11.0)
                .color(AnimrusTheme::TEXT_SECONDARY),
        );
    }
}
