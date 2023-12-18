// Internal
use super::MusicBox;
use crate::music::note::Note;
use crate::prelude::*;

impl MusicBox {
    pub fn is_valid_note(&self, note: &Note) -> bool {
        self.notes.contains(note)
    }
    pub fn get_scale_factor_y(&self) -> f64 {
        self.strip_height_mm / self.notes.len() as f64
    }

    pub fn note_count(&self) -> usize {
        self.notes.len()
    }
}
