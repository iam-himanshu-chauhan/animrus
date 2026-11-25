/// using Tile based rendering system - sparse tile algorithm
use std::collections::{HashMap, HashSet};

///tile dimension
pub const TILE_SIZE: u32 = 256;

///RGBA channels
pub const TILE_CHANNELS: usize = 4;

pub struct Tile {
    ///flat pixel buffer storing in format [R,G,B,A, R,G,B,A....]
    pixels: Vec<u8>,
}

impl Tile {
    pub fn new() -> Self {
        Self {
            pixels: vec![0; (TILE_SIZE * TILE_SIZE) as usize * TILE_CHANNELS],
        }
    }

    pub fn get_pixels(&self) -> &[u8] {
        &self.pixels
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
    tiles: HashMap<(u32, u32), Tile>,
    dirty_tiles: HashSet<(u32, u32)>,
}

impl RasterData {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            tiles: HashMap::new(),
            dirty_tiles: HashSet::new(),
        }
    }

    fn get_or_create_tile(&mut self, pixel_x: u32, pixel_y: u32) -> Option<&mut Tile> {
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

            self.dirty_tiles
                .insert((pixel_x / TILE_SIZE, pixel_y / TILE_SIZE));
        }
    }

    pub fn clear_tile(&mut self, tile_x: u32, tile_y: u32) {
        self.tiles.remove(&(tile_x, tile_y));
        self.mark_tile_dirty(tile_x, tile_y);
    }

    pub fn clear(&mut self) {
        for tile_coord in self.tiles.keys() {
            self.dirty_tiles.insert(*tile_coord);
        }

        self.tiles.clear();
    }

    pub fn is_tile_allocated(&self, tile_x: u32, tile_y: u32) -> bool {
        self.tiles.contains_key(&(tile_x, tile_y))
    }

    pub fn get_tile(&self, tile_x: u32, tile_y: u32) -> Option<&Tile> {
        self.tiles.get(&(tile_x, tile_y))
    }

    ///Count the number of tiles
    pub fn allocated_tile_count(&self) -> usize {
        self.tiles.len()
    }

    ///  Gets the allocated tiles
    pub fn allocated_tiles(&self) -> impl Iterator<Item = (u32, u32)> + '_ {
        self.tiles.keys().copied()
    }

    /// Returns the coordinates of all dirty tiles.
    pub fn dirty_tiles(&self) -> impl Iterator<Item = (u32, u32)> + '_ {
        self.dirty_tiles.iter().copied()
    }

    /// Marks a tile as dirty.
    pub fn mark_tile_dirty(&mut self, tile_x: u32, tile_y: u32) {
        self.dirty_tiles.insert((tile_x, tile_y));
    }

    /// Removes one tile from the dirty collection.
    pub fn clear_dirty_tile(&mut self, tile_x: u32, tile_y: u32) {
        self.dirty_tiles.remove(&(tile_x, tile_y));
    }

    /// Removes all dirty tile entries.
    pub fn clear_all_dirty_tiles(&mut self) {
        self.dirty_tiles.clear();
    }
}
