pub struct DocumentSettings{
    pub width: u32,
    pub height: u32,
    pub frame_rate: u32
}

impl DocumentSettings{
    pub fn new(width: u32, height: u32, frame_rate: u32) -> Self {
        Self {
            width,
            height,
            frame_rate,
        }
    }
}