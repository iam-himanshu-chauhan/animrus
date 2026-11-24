use crate::{dochandler::DocumentSettings, rendering::raster::RasterData};

/// Represents a layer within a drawing.
pub struct Layer {
    pub id: u64,
    pub name: String,
    pub visible: bool,
    pub opacity: f32,
    pub raster: RasterData,
}

impl Layer {
    pub fn new(id: u64, name: String, settings: &DocumentSettings) -> Self {
        Self {
            id,
            name,
            visible: true,
            opacity: 1.0,
            raster: RasterData::new(settings.width, settings.height),
        }
    }
}
