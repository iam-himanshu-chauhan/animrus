use animrus::rendering::raster::RasterData;

#[test]
fn pixel_access_across_tile_boundaries() {
    let mut raster = RasterData::new(512, 512);

    let red = [255, 0, 0, 255];
    let blue = [0, 0, 255, 255];

    raster.set_pixel(255, 255, red);
    raster.set_pixel(256, 256, blue);

    assert_eq!(raster.get_pixel(255, 255), Some(red));
    assert_eq!(raster.get_pixel(256, 256), Some(blue));

    assert_eq!(raster.tiles.len(), 2);
}

#[test]
fn pixel_access_outside_canvas() {
    let mut raster = RasterData::new(512, 512);

    let red = [255, 0, 0, 255];

    raster.set_pixel(512, 100, red);
    raster.set_pixel(100, 512, red);

    assert_eq!(raster.get_pixel(512, 100), None);
    assert_eq!(raster.get_pixel(100, 512), None);

    assert_eq!(raster.tiles.len(), 0);
}
