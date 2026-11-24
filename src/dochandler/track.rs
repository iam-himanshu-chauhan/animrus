use super::drawing::Drawing;

// holds drawing data
pub struct Track {
    pub id: u64,
    pub name: String,
    pub drawings: Vec<Drawing>,
}

impl Track {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            drawings: Vec::new(),
        }
    }

    pub fn add_drawing(&mut self, drawing: Drawing) {
        self.drawings.push(drawing);
    }

    pub fn get_drawing(&self, id: u64) -> Option<&Drawing> {
        self.drawings.iter().find(|c| c.id == id)
    }

    pub fn remove_drawing(&mut self, id: u64) {
        self.drawings.retain(|c| c.id != id);
    }
}
