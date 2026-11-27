use animrus::{
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
}

impl AnimrusApp {
    pub fn new() -> Self {
        // 1. Create document settings
        let settings = DocumentSettings::new(512, 512, 24);

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

        let mut app = Self {
            document,
            compositor_cache: CompositorCache::new(),
            canvas_texture: None,
        };

        app.update_compositor();

        app
    }

    fn update_compositor(&mut self) {
        let drawing = &mut self.document.compositions[0].tracks[0].drawings[0];

        self.compositor_cache
            .update(drawing, &self.document.settings);
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
        let ctx = ui.ctx().clone();

        if self.canvas_texture.is_none() {
            self.update_canvas_texture(&ctx);
        }

        ui.heading("Animrus Canvas");
        ui.separator();

        if let Some(texture) = &self.canvas_texture {
            let available_size = ui.available_size();
            let image_size = texture.size_vec2();

            let scale = (available_size.x / image_size.x)
                .min(available_size.y / image_size.y)
                .min(1.0);

            let display_size = image_size * scale;

            ui.centered_and_justified(|ui| {
                ui.image((texture.id(), display_size));
            });
        }
    }
}
