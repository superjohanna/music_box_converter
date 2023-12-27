// Internal
use super::MusicBox;
use crate::music::note::Note;
use crate::prelude::*;

impl MusicBox {
    pub fn is_valid_note(&self, note: &Option<Note>) -> bool {
        if note.is_none() {
            return false;
        }
        self.notes.contains(note.as_ref().unwrap())
    }
    pub fn vertical_note_distance(&self) -> f64 {
        self.strip_height_mm / self.notes.len() as f64
    }

    pub fn note_count(&self) -> usize {
        self.notes.len()
    }

    pub fn get_index(&self, note: &Note) -> Option<usize> {
        self.notes.iter().position(|el| *el == *note)
    }
}
