use crate::{
    cache_sys::cache::Cache,
    dochandler::{DocumentSettings, Drawing},
    rendering::compositor::{collect_dirty_tiles, composite_dirty_tiles},
};

/// Stores composited tile pixel buffers.
pub struct CompositorCache {
    tiles: Cache<(u64, u32, u32), Vec<u8>>,
}

impl CompositorCache {
    pub fn new() -> Self {
        Self {
            tiles: Cache::new(),
        }
    }

    /// Recomputes dirty tiles and updates their cached results.
    pub fn update(&mut self, drawing: &mut Drawing, settings: &DocumentSettings) -> bool {
        let dirty_tiles = collect_dirty_tiles(drawing);

        if dirty_tiles.is_empty() {
            return false;
        }

        let updated_tiles = composite_dirty_tiles(drawing, settings);

        for ((tile_x, tile_y), pixels) in updated_tiles {
            self.tiles.insert((drawing.id, tile_x, tile_y), pixels);
        }

        // All dirty tiles have now been recomposited.
        for layer in &mut drawing.layers {
            layer.raster.clear_all_dirty_tiles();
        }

        drawing.clear_composite_dirty_tiles();
        true
    }

    /// Gets a cached composite tile.
    pub fn get_tile(&self, drawing_id: u64, tile_x: u32, tile_y: u32) -> Option<&[u8]> {
        self.tiles
            .get(&(drawing_id, tile_x, tile_y))
            .map(Vec::as_slice)
    }

    pub fn clear(&mut self) {
        self.tiles.clear();
    }

    pub fn len(&self) -> usize {
        self.tiles.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tiles.is_empty()
    }
}

impl Default for CompositorCache {
    fn default() -> Self {
        Self::new()
    }
}
