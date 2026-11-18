mod composition;
mod drawing;
mod layer;
mod settings;
mod track;

use composition::Composition;
use settings::DocumentSettings;

pub struct AniRusDocument {
    pub settings: DocumentSettings,
    pub compositions: Vec<Composition>,
}

impl AniRusDocument {
    pub fn new(settings: DocumentSettings) -> Self {
        Self {
            settings,
            compositions: Vec::new(),
        }
    }

    pub fn add_composition(&mut self, composition: Composition) {
        self.compositions.push(composition);
    }

    pub fn get_composition(&self, id: u64) -> Option<&Composition> {
        self.compositions.iter().find(|c| c.id == id)
    }

    pub fn remove_composition(&mut self, id: u64) {
        self.compositions.retain(|c| c.id != id);
    }
}
