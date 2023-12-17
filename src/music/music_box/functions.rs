// Internal
use super::MusicBox;
use crate::music::note::Note;
use crate::prelude::*;

impl MusicBox {
    pub fn is_valid_note(&self, note: &Note) -> bool {
        self.notes.contains(note)
    }
    pub fn get_scale_factor_y(&self) -> f64 {
        self.min_note_distance_mm / self.notes.len() as f64
    }
}
