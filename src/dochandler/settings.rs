/// contains the document settings
pub struct DocumentSettings {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub frame_rate: u32,
}

impl DocumentSettings {
    pub fn new(name: String, width: u32, height: u32, frame_rate: u32) -> Self {
        Self {
            name,
            width,
            height,
            frame_rate,
        }
    }
}
