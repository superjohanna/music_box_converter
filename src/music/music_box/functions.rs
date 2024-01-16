// Internal
use super::MusicBox;
use crate::music::note::Note;
use crate::prelude::*;

impl MusicBox {
    pub fn is_valid_note(&self, note: &Note) -> bool {
        self.notes.contains(note)
    }
    pub fn vertical_note_distance(&self) -> f64 {
        self.strip_height_mm / (self.notes.len() as f64 - 1f64)
    }

    pub fn note_count(&self) -> usize {
        self.notes.len()
    }

    pub fn get_index(&self, note: &Note) -> Option<usize> {
        self.notes.iter().position(|el| *el == *note)
    }
}
