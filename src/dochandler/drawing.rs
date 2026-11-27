use std::collections::HashSet;

use super::layer::Layer;

/// Contains artwork and defines when it appears on the timeline.
pub struct Drawing {
    pub id: u64,
    pub name: String,
    pub start_frame: u32,
    pub duration: u32,
    pub layers: Vec<Layer>,
    composite_dirty_tiles: HashSet<(u32, u32)>,
}

impl Drawing {
    pub fn new(id: u64, name: String, start_frame: u32, duration: u32) -> Self {
        Self {
            id,
            name,
            start_frame,
            duration,
            layers: Vec::new(),
            composite_dirty_tiles: HashSet::new(),
        }
    }

    pub fn add_layer(&mut self, layer: Layer) {
        let dirty_tiles: Vec<_> = layer.raster.allocated_tiles().collect();

        self.layers.push(layer);

        if !dirty_tiles.is_empty() {
            for (tile_x, tile_y) in dirty_tiles {
                self.mark_composite_dirty(tile_x, tile_y);
            }
        }
    }

    pub fn get_layer(&self, id: u64) -> Option<&Layer> {
        self.layers.iter().find(|c| c.id == id)
    }

    pub fn get_mut_layer(&mut self, id: u64) -> Option<&mut Layer> {
        self.layers.iter_mut().find(|c| c.id == id)
    }

    pub fn remove_layer(&mut self, id: u64) {
        if let Some(index) = self.layers.iter().position(|layer| layer.id == id) {
            let dirty_tiles: Vec<_> = self.layers[index].raster.allocated_tiles().collect();

            self.layers.remove(index);

            for (tile_x, tile_y) in dirty_tiles {
                self.mark_composite_dirty(tile_x, tile_y);
            }
        }
    }

    pub fn mark_composite_dirty(&mut self, tile_x: u32, tile_y: u32) {
        self.composite_dirty_tiles.insert((tile_x, tile_y));
    }

    pub fn composite_dirty_tiles(&self) -> impl Iterator<Item = (u32, u32)> + '_ {
        self.composite_dirty_tiles.iter().copied()
    }

    pub fn clear_composite_dirty_tiles(&mut self) {
        self.composite_dirty_tiles.clear();
    }
}
