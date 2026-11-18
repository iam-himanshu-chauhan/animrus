use super::layer::Layer;

/// Contains artwork and defines when it appears on the timeline.
pub struct Drawing {
    pub id: u64,
    pub name: String,
    pub start_frame: u32,
    pub duration: u32,
    pub layers: Vec<Layer>,
}

impl Drawing {
    pub fn new(id: u64, name: String, start_frame: u32, duration: u32) -> Self {
        Self {
            id,
            name,
            start_frame,
            duration,
            layers: Vec::new(),
        }
    }

    pub fn add_layer(&mut self, layer: Layer) {
        self.layers.push(layer);
    }

    pub fn get_layer(&self, id: u64) -> Option<&Layer> {
        self.layers.iter().find(|c| c.id == id)
    }

    pub fn remove_layer(&mut self, id: u64) {
        self.layers.retain(|c| c.id != id);
    }
}
