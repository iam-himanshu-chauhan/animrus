use std::sync::Arc;

use egui::{Color32, FontData, FontDefinitions, FontFamily, FontId, Stroke, TextStyle, Vec2};

pub struct AnimrusTheme;

impl AnimrusTheme {
    // Main backgrounds
    pub const BACKGROUND: Color32 = Color32::from_rgb(24, 25, 29);
    pub const PANEL: Color32 = Color32::from_rgb(32, 33, 39);
    pub const SURFACE: Color32 = Color32::from_rgb(40, 42, 50);

    // Borders
    pub const BORDER: Color32 = Color32::from_rgb(53, 55, 65);

    // Text
    pub const TEXT_PRIMARY: Color32 = Color32::from_rgb(232, 234, 240);
    pub const TEXT_SECONDARY: Color32 = Color32::from_rgb(146, 150, 163);
    pub const TEXT_DISABLED: Color32 = Color32::from_rgb(98, 102, 115);

    // Accent
    pub const ACCENT: Color32 = Color32::from_rgb(130, 152, 255);
    pub const SELECTION: Color32 = Color32::from_rgb(53, 71, 121);

    // Spacing
    pub const PANEL_PADDING: f32 = 8.0;
    pub const ITEM_SPACING: f32 = 6.0;

    pub fn apply(ctx: &egui::Context) {
        Self::apply_fonts(ctx);
        Self::apply_visuals(ctx);
        Self::apply_spacing(ctx);
    }

    fn apply_fonts(ctx: &egui::Context) {
        let mut fonts = FontDefinitions::default();

        // Register Geist fonts.
        fonts.font_data.insert(
            "geist_regular".to_owned(),
            Arc::new(FontData::from_static(include_bytes!(
                "../../assets/fonts/Geist-Regular.ttf"
            ))),
        );

        fonts.font_data.insert(
            "geist_medium".to_owned(),
            Arc::new(FontData::from_static(include_bytes!(
                "../../assets/fonts/Geist-Medium.ttf"
            ))),
        );

        fonts.font_data.insert(
            "geist_semibold".to_owned(),
            Arc::new(FontData::from_static(include_bytes!(
                "../../assets/fonts/Geist-SemiBold.ttf"
            ))),
        );

        fonts.font_data.insert(
            "geist_bold".to_owned(),
            Arc::new(FontData::from_static(include_bytes!(
                "../../assets/fonts/Geist-Bold.ttf"
            ))),
        );

        let regular = FontFamily::Name(Arc::from("Geist"));
        let medium = FontFamily::Name(Arc::from("Geist Medium"));
        let semibold = FontFamily::Name(Arc::from("Geist SemiBold"));
        let bold = FontFamily::Name(Arc::from("Geist Bold"));

        fonts
            .families
            .insert(regular.clone(), vec!["geist_regular".to_owned()]);

        fonts
            .families
            .insert(medium.clone(), vec!["geist_medium".to_owned()]);

        fonts
            .families
            .insert(semibold.clone(), vec!["geist_semibold".to_owned()]);

        fonts
            .families
            .insert(bold.clone(), vec!["geist_bold".to_owned()]);

        // Use Geist as the default proportional font.
        fonts
            .families
            .insert(FontFamily::Proportional, vec!["geist_regular".to_owned()]);

        ctx.set_fonts(fonts);

        // Define typography hierarchy.
        let mut style = (*ctx.global_style()).clone();
        style
            .text_styles
            .insert(TextStyle::Body, FontId::new(13.0, regular.clone()));

        style
            .text_styles
            .insert(TextStyle::Button, FontId::new(13.0, medium));

        style
            .text_styles
            .insert(TextStyle::Heading, FontId::new(18.0, semibold));

        style
            .text_styles
            .insert(TextStyle::Small, FontId::new(11.0, regular));

        style
            .text_styles
            .insert(TextStyle::Monospace, FontId::new(12.0, bold));

        ctx.set_global_style(style);
    }

    fn apply_visuals(ctx: &egui::Context) {
        let mut visuals = egui::Visuals::dark();

        visuals.panel_fill = Self::PANEL;
        visuals.window_fill = Self::PANEL;
        visuals.extreme_bg_color = Self::BACKGROUND;
        visuals.faint_bg_color = Self::SURFACE;

        visuals.window_stroke = Stroke::new(1.0, Self::BORDER);

        // Normal widgets
        visuals.widgets.noninteractive.bg_fill = Self::PANEL;
        visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, Self::TEXT_PRIMARY);

        // Inactive widgets
        visuals.widgets.inactive.bg_fill = Self::SURFACE;
        visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, Self::TEXT_PRIMARY);

        // Hovered widgets
        visuals.widgets.hovered.bg_fill = Color32::from_rgb(55, 58, 69);
        visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, Color32::WHITE);

        // Active widgets
        visuals.widgets.active.bg_fill = Self::SELECTION;
        visuals.widgets.active.fg_stroke = Stroke::new(1.0, Self::TEXT_PRIMARY);

        // Open widgets
        visuals.widgets.open.bg_fill = Self::SURFACE;
        visuals.widgets.open.fg_stroke = Stroke::new(1.0, Self::TEXT_PRIMARY);

        visuals.selection.bg_fill = Self::SELECTION;
        visuals.selection.stroke = Stroke::new(1.0, Self::ACCENT);

        ctx.set_visuals(visuals);
    }

    fn apply_spacing(ctx: &egui::Context) {
        let mut style = (*ctx.global_style()).clone();

        style.spacing.item_spacing = Vec2::new(Self::ITEM_SPACING, Self::ITEM_SPACING);

        style.spacing.button_padding = Vec2::new(10.0, 5.0);
        style.spacing.interact_size = Vec2::new(24.0, 24.0);

        ctx.set_global_style(style);
    }
}
