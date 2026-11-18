use super::drawing::Drawing;

// holds drawing data
pub struct Track {
    pub id: u64,
    pub name: String,
    pub drawings: Vec<Drawing>
}
