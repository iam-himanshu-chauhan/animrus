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

    assert_eq!(raster.allocated_tile_count(), 2);
}

#[test]
fn pixel_access_outside_canvas() {
    let mut raster = RasterData::new(512, 512);

    let red = [255, 0, 0, 255];

    raster.set_pixel(512, 100, red);
    raster.set_pixel(100, 512, red);

    assert_eq!(raster.get_pixel(512, 100), None);
    assert_eq!(raster.get_pixel(100, 512), None);

    assert_eq!(raster.allocated_tile_count(), 0);
}

#[test]
fn clear_tile_removes_allocated_tile() {
    let mut raster = RasterData::new(512, 512);

    raster.set_pixel(300, 300, [255, 0, 0, 255]);

    assert_eq!(raster.allocated_tile_count(), 1);

    raster.clear_tile(1, 1);

    assert_eq!(raster.allocated_tile_count(), 0);
    assert_eq!(raster.get_pixel(300, 300), Some([0, 0, 0, 0]));
}

#[test]
fn clear_removes_all_tiles() {
    let mut raster = RasterData::new(512, 512);

    raster.set_pixel(100, 100, [255, 0, 0, 255]);
    raster.set_pixel(300, 300, [0, 0, 255, 255]);

    assert_eq!(raster.allocated_tile_count(), 2);

    raster.clear();

    assert_eq!(raster.allocated_tile_count(), 0);
    assert_eq!(raster.get_pixel(100, 100), Some([0, 0, 0, 0]));
}

#[test]
fn check_tile_allocation() {
    let mut raster = RasterData::new(512, 512);

    assert!(!raster.is_tile_allocated(1, 1));

    raster.set_pixel(300, 300, [255, 0, 0, 255]);

    assert!(raster.is_tile_allocated(1, 1));
    assert!(!raster.is_tile_allocated(0, 0));
}

#[test]
fn get_allocated_tile() {
    let mut raster = RasterData::new(512, 512);

    raster.set_pixel(300, 300, [255, 0, 0, 255]);

    let tile = raster.get_tile(1, 1);

    assert!(tile.is_some());
    assert!(raster.get_tile(0, 0).is_none());
}
