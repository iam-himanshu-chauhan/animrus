use std::collections::{HashMap, HashSet};

use crate::dochandler::{DocumentSettings, Drawing};
use crate::rendering::raster::{TILE_CHANNELS, TILE_SIZE};

/// Combines one pixel across layers.
pub fn composite_pixels(drawing: &Drawing, x: u32, y: u32) -> [u8; 4] {
    let mut result = [0u8; 4];

    for layer in &drawing.layers {
        if !layer.visible || layer.opacity <= 0.0 {
            continue;
        }

        if let Some(mut source) = layer.raster.get_pixel(x, y) {
            source[3] = (source[3] as f32 * layer.opacity.clamp(0.0, 1.0)).round() as u8;

            if source[3] == 0 {
                continue;
            }

            result = blend_over(result, source);
        }
    }

    result
}

/// Composites a source pixel over a destination pixel using
/// Normal blending (Porter-Duff Source-Over).
fn blend_over(destination: [u8; 4], source: [u8; 4]) -> [u8; 4] {
    let src_alpha = source[3] as f32 / 255.0;
    let dst_alpha = destination[3] as f32 / 255.0;

    let out_alpha = src_alpha + dst_alpha * (1.0 - src_alpha);

    if out_alpha == 0.0 {
        return [0, 0, 0, 0];
    }

    let mut output = [0u8; 4];

    for channel in 0..3 {
        let src = source[channel] as f32 / 255.0;
        let dst = destination[channel] as f32 / 255.0;

        let out = (src * src_alpha + dst * dst_alpha * (1.0 - src_alpha)) / out_alpha;

        output[channel] = (out * 255.0).round() as u8;
    }

    output[3] = (out_alpha * 255.0).round() as u8;

    output
}

/// Composites one tile across all visible layers.
pub fn composite_tile(
    drawing: &Drawing,
    tile_x: u32,
    tile_y: u32,
    settings: &DocumentSettings,
) -> Vec<u8> {
    let width = settings.width;
    let height = settings.height;

    let start_x = tile_x * TILE_SIZE;
    let start_y = tile_y * TILE_SIZE;

    // Reject tiles outside the canvas.
    if start_x >= width || start_y >= height {
        return Vec::new();
    }

    let end_x = start_x.saturating_add(TILE_SIZE).min(width);
    let end_y = start_y.saturating_add(TILE_SIZE).min(height);

    let tile_width = end_x - start_x;
    let tile_height = end_y - start_y;

    let mut output = vec![0u8; tile_width as usize * tile_height as usize * 4];

    // Collect the corresponding tile from each visible layer.
    let mut layer_tiles = Vec::new();

    for layer in &drawing.layers {
        if !layer.visible || layer.opacity <= 0.0 {
            continue;
        }

        if let Some(tile) = layer.raster.get_tile(tile_x, tile_y) {
            layer_tiles.push((tile.get_pixels(), layer.opacity.clamp(0.0, 1.0)));
        }
    }

    // Composite pixels inside this tile.
    for y in start_y..end_y {
        for x in start_x..end_x {
            let local_x = x - start_x;
            let local_y = y - start_y;

            let tile_index = ((local_y * TILE_SIZE + local_x) as usize) * TILE_CHANNELS;

            let mut result = [0u8; 4];

            for (pixels, opacity) in &layer_tiles {
                let mut source: [u8; 4] = pixels[tile_index..tile_index + 4].try_into().unwrap();

                source[3] = (source[3] as f32 * opacity).round() as u8;

                if source[3] == 0 {
                    continue;
                }

                result = blend_over(result, source);
            }

            let output_index = ((local_y * tile_width + local_x) as usize) * 4;

            output[output_index..output_index + 4].copy_from_slice(&result);
        }
    }

    output
}

/// Combines all layers inside one Drawing into a full RGBA pixel buffer.
pub fn composite_drawing(drawing: &Drawing, settings: &DocumentSettings) -> Vec<u8> {
    let width = settings.width;
    let height = settings.height;

    let mut output = vec![0u8; width as usize * height as usize * 4];

    // Collect unique occupied tiles.
    let mut occupied_tiles = HashSet::new();

    for layer in &drawing.layers {
        if !layer.visible || layer.opacity <= 0.0 {
            continue;
        }

        for tile_coord in layer.raster.allocated_tiles() {
            occupied_tiles.insert(tile_coord);
        }
    }

    // Composite each occupied tile.
    for (tile_x, tile_y) in occupied_tiles {
        let tile_pixels = composite_tile(drawing, tile_x, tile_y, settings);

        let start_x = tile_x * TILE_SIZE;
        let start_y = tile_y * TILE_SIZE;

        let tile_width = TILE_SIZE.min(width - start_x);
        let tile_height = TILE_SIZE.min(height - start_y);

        // Copy tile pixels into the full canvas buffer.
        for y in 0..tile_height {
            for x in 0..tile_width {
                let tile_index = ((y * tile_width + x) as usize) * 4;

                let output_index = (((start_y + y) * width + (start_x + x)) as usize) * 4;

                output[output_index..output_index + 4]
                    .copy_from_slice(&tile_pixels[tile_index..tile_index + 4]);
            }
        }
    }

    output
}

pub fn collect_dirty_tiles(drawing: &Drawing) -> HashSet<(u32, u32)> {
    let mut collected_tiles = HashSet::new();
    for layer in &drawing.layers {
        for tile in layer.raster.dirty_tiles() {
            collected_tiles.insert(tile);
        }
    }

    for tile in drawing.composite_dirty_tiles() {
        collected_tiles.insert(tile);
    }
    collected_tiles
}

pub fn composite_dirty_tiles(
    drawing: &Drawing,
    settings: &DocumentSettings,
) -> HashMap<(u32, u32), Vec<u8>> {
    let tiles = collect_dirty_tiles(drawing);
    let mut output = HashMap::new();

    for tile in tiles {
        output.insert(tile, composite_tile(drawing, tile.0, tile.1, settings));
    }

    output
}
