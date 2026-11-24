use super::track::Track;

/// contains tracks
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

    pub fn add_track(&mut self, track: Track) {
        self.tracks.push(track);
    }

    pub fn get_track(&self, id: u64) -> Option<&Track> {
        self.tracks.iter().find(|t| t.id == id)
    }

    pub fn remove_track(&mut self, id: u64) {
        self.tracks.retain(|t| t.id != id);
    }
}