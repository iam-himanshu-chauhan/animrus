use animrus::{
    dochandler::{DocumentSettings, Drawing, Layer},
    rendering::compositor::{composite_drawing, composite_pixels},
};

#[test]
fn composites_single_opaque_layer() {
    let settings = DocumentSettings::new(1920, 1080, 24);
    let mut drawing = Drawing::new(0, "Test".to_string(), 0, 1);

    drawing.add_layer(Layer::new(32, "Red".to_string(), &settings));

    drawing
        .get_mut_layer(32)
        .unwrap()
        .raster
        .set_pixel(10, 10, [255, 0, 0, 255]);

    assert_eq!(composite_pixels(&drawing, 10, 10), [255, 0, 0, 255]);
}

#[test]
fn composites_semitransparent_top_layer() {
    let settings = DocumentSettings::new(1920, 1080, 24);
    let mut drawing = Drawing::new(0, "Test".to_string(), 0, 1);

    drawing.add_layer(Layer::new(32, "Red".to_string(), &settings));
    drawing
        .get_mut_layer(32)
        .unwrap()
        .raster
        .set_pixel(10, 10, [255, 0, 0, 255]);

    drawing.add_layer(Layer::new(33, "Blue".to_string(), &settings));
    drawing
        .get_mut_layer(33)
        .unwrap()
        .raster
        .set_pixel(10, 10, [0, 0, 255, 128]);

    assert_eq!(composite_pixels(&drawing, 10, 10), [127, 0, 128, 255]);
}

#[test]
fn hidden_layer_is_not_composited() {
    let settings = DocumentSettings::new(1920, 1080, 24);
    let mut drawing = Drawing::new(0, "Test".to_string(), 0, 1);

    drawing.add_layer(Layer::new(32, "Red".to_string(), &settings));
    drawing
        .get_mut_layer(32)
        .unwrap()
        .raster
        .set_pixel(10, 10, [255, 0, 0, 255]);

    drawing.add_layer(Layer::new(33, "Blue".to_string(), &settings));
    drawing
        .get_mut_layer(33)
        .unwrap()
        .raster
        .set_pixel(10, 10, [0, 0, 255, 255]);

    drawing.get_mut_layer(32).unwrap().visible = false;

    assert_eq!(composite_pixels(&drawing, 10, 10), [0, 0, 255, 255]);
}

#[test]
fn optimized_compositor_matches_pixel_compositor() {
    let settings = DocumentSettings::new(260, 258, 24);

    let mut drawing = Drawing::new(1, "Test Drawing".to_string(), 0, 1);

    // Bottom layer: opaque red pixels.
    let mut bottom = Layer::new(1, "Bottom".to_string(), &settings);
    bottom.raster.set_pixel(0, 0, [255, 0, 0, 255]);
    bottom.raster.set_pixel(255, 255, [255, 0, 0, 255]);
    bottom.raster.set_pixel(256, 256, [255, 0, 0, 255]);
    bottom.raster.set_pixel(259, 257, [255, 0, 0, 255]);

    // Top layer: partially transparent blue pixels.
    let mut top = Layer::new(2, "Top".to_string(), &settings);
    top.opacity = 0.5;
    top.raster.set_pixel(0, 0, [0, 0, 255, 255]);
    top.raster.set_pixel(256, 256, [0, 0, 255, 128]);

    // Hidden layer: should not affect the result.
    let mut hidden = Layer::new(3, "Hidden".to_string(), &settings);
    hidden.visible = false;
    hidden.raster.set_pixel(0, 0, [0, 255, 0, 255]);

    drawing.add_layer(bottom);
    drawing.add_layer(top);
    drawing.add_layer(hidden);

    // Result from the optimized implementation.
    let optimized = composite_drawing(&drawing, &settings);

    // Reference result: composite one pixel at a time.
    let mut reference = Vec::new();

    for y in 0..settings.height {
        for x in 0..settings.width {
            reference.extend_from_slice(&composite_pixels(&drawing, x, y));
        }
    }

    assert_eq!(optimized, reference);
}
