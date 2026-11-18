use super::track::Track;


pub struct Composition {
    pub id: u64, // simple incremental counter
    pub name: String,
    pub tracks: Vec<Track>
}

impl Composition {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            tracks: Vec::new()
        }

    }
}