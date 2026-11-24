use animrus::dochandler::{DocumentSettings, Layer};

#[test]
fn layer_creation_test() {
    let docsettings = DocumentSettings::new(1920, 1080, 24);

    let layer = Layer::new(0, "mylayer".to_string(), &docsettings);


    assert_eq!(layer.raster.width, 1920);
    assert_eq!(layer.raster.height, 1080);
    
}
