/// using Tile based rendering system - sparse tile algorithm
use std::collections::HashMap;

///tile dimension
pub const TILE_SIZE: u32 = 256;

///RGBA channels
pub const TILE_CHANNELS: usize = 4;

pub struct Tile {
    ///flat pixel buffer storing in format [R,G,B,A, R,G,B,A....]
    pub pixels: Vec<u8>,
}

impl Tile {
    pub fn new() -> Self {
        Self {
            pixels: vec![0; (TILE_SIZE * TILE_SIZE) as usize * TILE_CHANNELS],
        }
    }
}

impl Default for Tile {
    fn default() -> Self {
        Self::new()
    }
}

///represents the raster artwork of one layer, not the entire drawing.
pub struct RasterData {
    /// canvas width
    pub width: u32,

    /// canvas height
    pub height: u32,

    ///grid coordinate of tile `(tile_x, tile_y)` <br/>
    /// increase `tile_x` = move left to right <br/>
    /// increase `tile_y` = move top to bottom
    pub tiles: HashMap<(u32, u32), Tile>,
}

impl RasterData {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            tiles: HashMap::new(),
        }
    }

    pub fn get_or_create_tile(&mut self, pixel_x: u32, pixel_y: u32) -> Option<&mut Tile> {
        if pixel_x >= self.width || pixel_y >= self.height {
            None
        } else {
            Some(
                self.tiles
                    .entry((pixel_x / TILE_SIZE, pixel_y / TILE_SIZE))
                    .or_default(),
            )
        }
    }

    /// Gets RGBA from pixel buffer
    pub fn get_pixel(&self, pixel_x: u32, pixel_y: u32) -> Option<[u8; 4]> {
        if pixel_x >= self.width || pixel_y >= self.height {
            return None;
        }

        // getting which tile holds the pixel
        let tile_x = pixel_x / TILE_SIZE;
        let tile_y = pixel_y / TILE_SIZE;

        // converting pixel coordinates to local tile coordinates
        let local_x = pixel_x % TILE_SIZE;
        let local_y = pixel_y % TILE_SIZE;

        let index = ((local_y * TILE_SIZE + local_x) * TILE_CHANNELS as u32) as usize;

        if let Some(arr) = self.tiles.get(&(tile_x, tile_y)) {
            let pixel: [u8; 4] = arr.pixels[index..index + 4].try_into().ok()?;
            return Some(pixel);
        }
        Some([0, 0, 0, 0])
    }

    /// Sets the RGBA channels in pixel buffer
    pub fn set_pixel(&mut self, pixel_x: u32, pixel_y: u32, color: [u8; 4]) {
        if pixel_x < self.width
            && pixel_y < self.height
            && let Some(tile) = self.get_or_create_tile(pixel_x, pixel_y)
        {
            let local_x = pixel_x % TILE_SIZE;
            let local_y = pixel_y % TILE_SIZE;

            let index = ((local_y * TILE_SIZE + local_x) * TILE_CHANNELS as u32) as usize;
            tile.pixels[index..index + 4].copy_from_slice(&color);
        }
    }
}
