use std::collections::HashSet;

use crate::dochandler::{DocumentSettings, Drawing};
use crate::rendering::raster::TILE_SIZE;

/// Combines one pixel across layers.
pub fn composite_pixels(drawing: &Drawing, x: u32, y: u32) -> [u8; 4] {
    let mut result = [0u8; 4];

    for layer in &drawing.layers {
        if !layer.visible {
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

/// Combines all layers inside one Drawing into a single RGBA pixel buffer.
pub fn composite_drawing(drawing: &Drawing, settings: &DocumentSettings) -> Vec<u8> {
    let width = settings.width;
    let height = settings.height;

    let total_bytes = width as usize * height as usize * 4;
    let mut output = vec![0u8; total_bytes];

    // Collect unique allocated tiles from visible layers.
    let mut occupied_tiles = HashSet::new();

    for layer in &drawing.layers {
        if !layer.visible {
            continue;
        }

        for tile_coord in layer.raster.allocated_tiles() {
            occupied_tiles.insert(tile_coord);
        }
    }

    // Process only occupied tiles.
    for (tile_x, tile_y) in occupied_tiles {
        let start_x = tile_x * TILE_SIZE;
        let start_y = tile_y * TILE_SIZE;

        let end_x = start_x.saturating_add(TILE_SIZE).min(width);
        let end_y = start_y.saturating_add(TILE_SIZE).min(height);

        for y in start_y..end_y {
            for x in start_x..end_x {
                let pixel = composite_pixels(drawing, x, y);

                let index = ((y as usize * width as usize) + x as usize) * 4;

                output[index..index + 4].copy_from_slice(&pixel);
            }
        }
    }

    output
}
