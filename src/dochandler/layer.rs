/// Represents a layer within a drawing.
pub struct Layer {
    pub id: u64,
    pub name: String,
    pub visible: bool,
    pub opacity: f32,
}

impl Layer {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            visible: true,
            opacity: 1.0,
        }
    }
}
