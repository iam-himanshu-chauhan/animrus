use crate::input_manager::{InputAction, InputBinding, InputManager};

use super::{
    dochandler::{AniRusDocument, Composition, DocumentSettings, Drawing, Layer, Track},
    rendering::compositor_cache::CompositorCache,
};

pub struct AnimrusApp {
    /// original project data
    document: AniRusDocument,

    /// cache of composited tiles
    compositor_cache: CompositorCache,

    /// holds GPU texture displayed by egui
    canvas_texture: Option<egui::TextureHandle>,
    input_manager: InputManager,
    canvas_zoom: f32,
    canvas_offset: egui::Vec2,
    canvas_zoom_action: InputAction,
    canvas_pan_action: InputAction,
    canvas_background: egui::Color32,
}

impl AnimrusApp {
    pub fn new(name: String) -> Self {
        // 1. Create document settings
        let settings = DocumentSettings::new(name, 512, 512, 24);

        // 2. Create document
        let mut document = AniRusDocument::new(settings);

        // 3. Create composition
        let mut composition = Composition::new(1, "Main Composition".to_string());

        // 4. Create track
        let mut track = Track::new(1, "Main Track".to_string());

        // 5. Create drawing
        let mut drawing = Drawing::new(1, "Drawing 1".to_string(), 0, 1);

        // 6. Create raster layer using document settings
        let mut layer = Layer::new(1, "Background".to_string(), &document.settings);

        // 7. Draw sample artwork
        draw_sample_artwork(&mut layer);

        // 8. Add everything to its parent
        drawing.add_layer(layer);
        track.add_drawing(drawing);
        composition.add_track(track);
        document.add_composition(composition);

        let canvas_zoom_action = InputAction::new("canvas.zoom");
        let canvas_pan_action = InputAction::new("canvas.pan");

        let mut local_input_manager = InputManager::new();

        local_input_manager.bind(canvas_zoom_action.clone(), InputBinding::MouseWheel);

        local_input_manager.bind(
            canvas_pan_action.clone(),
            InputBinding::MouseButton(egui::PointerButton::Middle),
        );

        let mut app = Self {
            document,
            compositor_cache: CompositorCache::new(),
            canvas_texture: None,
            input_manager: local_input_manager,
            canvas_zoom_action,
            canvas_pan_action,
            canvas_zoom: 1.0,
            canvas_offset: egui::Vec2::ZERO,
            canvas_background: egui::Color32::from_rgb(45, 45, 45),
        };

        app.update_compositor();

        app
    }

    fn update_compositor(&mut self) -> bool {
        let drawing = &mut self.document.compositions[0].tracks[0].drawings[0];

        self.compositor_cache
            .update(drawing, &self.document.settings)
    }

    fn build_canvas_image(&self) -> egui::ColorImage {
        let width = self.document.settings.width;
        let height = self.document.settings.height;

        let mut pixels = vec![egui::Color32::TRANSPARENT; (width * height) as usize];

        let drawing = &self.document.compositions[0].tracks[0].drawings[0];

        for tile_y in 0..height.div_ceil(256) {
            for tile_x in 0..width.div_ceil(256) {
                let Some(tile) = self.compositor_cache.get_tile(drawing.id, tile_x, tile_y) else {
                    continue;
                };

                let tile_width = 256.min(width - tile_x * 256);
                let tile_height = 256.min(height - tile_y * 256);

                for y in 0..tile_height {
                    for x in 0..tile_width {
                        let tile_index = ((y * tile_width + x) * 4) as usize;

                        let canvas_index =
                            (((tile_y * 256 + y) * width) + (tile_x * 256 + x)) as usize;

                        pixels[canvas_index] = egui::Color32::from_rgba_unmultiplied(
                            tile[tile_index],
                            tile[tile_index + 1],
                            tile[tile_index + 2],
                            tile[tile_index + 3],
                        );
                    }
                }
            }
        }

        egui::ColorImage {
            size: [width as usize, height as usize],
            source_size: egui::vec2(width as f32, height as f32),
            pixels,
        }
    }

    fn update_canvas_texture(&mut self, ctx: &egui::Context) {
        let image = self.build_canvas_image();

        if let Some(texture) = &mut self.canvas_texture {
            texture.set(image, egui::TextureOptions::NEAREST);
        } else {
            self.canvas_texture =
                Some(ctx.load_texture("animrus_canvas", image, egui::TextureOptions::NEAREST));
        }
    }
}

fn draw_sample_artwork(layer: &mut Layer) {
    let center_x = 256;
    let center_y = 256;
    let radius = 100;

    for y in 0..512 {
        for x in 0..512 {
            let dx = x as i32 - center_x;
            let dy = y as i32 - center_y;

            if dx * dx + dy * dy <= radius * radius {
                layer.raster.set_pixel(x, y, [50, 160, 255, 255]);
            }
        }
    }
}

impl eframe::App for AnimrusApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.input_manager.update(ui.ctx());

        let ctx = ui.ctx().clone();
        let canvas_changed = self.update_compositor();

        if canvas_changed || self.canvas_texture.is_none() {
            self.update_canvas_texture(&ctx);
        }

        ui.heading("Animrus Canvas");
        ui.separator();

        if ui.button("Change Canvas").clicked() {
            let drawing = &mut self.document.compositions[0].tracks[0].drawings[0];
            let layer = &mut drawing.layers[0];

            for y in 200..300 {
                for x in 200..300 {
                    layer.raster.set_pixel(x, y, [255, 0, 0, 255]);
                }
            }

            ui.ctx().request_repaint();
        }

        let available_size = ui.available_size();

        let (viewport_rect, _response) =
            ui.allocate_exact_size(available_size, egui::Sense::hover());

        // Background
        ui.painter()
            .rect_filled(viewport_rect, 0.0, self.canvas_background);

        // Zoom using Input Manager
        let pointer_pos = ui.input(|i| i.pointer.hover_pos());

        let zoom_delta = self.input_manager.action_value(&self.canvas_zoom_action).y;

        if let Some(pointer_pos) = pointer_pos
            && viewport_rect.contains(pointer_pos)
            && zoom_delta != 0.0
        {
            let old_zoom = self.canvas_zoom;

            self.canvas_zoom = (old_zoom * (zoom_delta * 0.01).exp()).clamp(0.1, 8.0);

            let zoom_ratio = self.canvas_zoom / old_zoom;

            self.canvas_offset = pointer_pos
                - viewport_rect.center()
                - (pointer_pos - viewport_rect.center() - self.canvas_offset) * zoom_ratio;
        }

        // Draw canvas
        if let Some(texture) = &self.canvas_texture {
            let image_size = texture.size_vec2();

            let fit_scale = (viewport_rect.width() / image_size.x)
                .min(viewport_rect.height() / image_size.y)
                .min(1.0);

            let display_size = image_size * fit_scale * self.canvas_zoom;

            let image_rect = egui::Rect::from_center_size(
                viewport_rect.center() + self.canvas_offset,
                display_size,
            );

            let painter = ui.painter().with_clip_rect(viewport_rect);

            // Draw the canvas surface behind the artwork.
            painter.rect_filled(image_rect, 0.0, egui::Color32::WHITE);

            // Draw the artwork.
            painter.image(
                texture.id(),
                image_rect,
                egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0)),
                egui::Color32::WHITE,
            );
        }
    }
}
