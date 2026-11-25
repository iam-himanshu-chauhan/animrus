use animrus::{
    dochandler::{DocumentSettings, Drawing, Layer},
    rendering::compositor::composite_pixels,
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
