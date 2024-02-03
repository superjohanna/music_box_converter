// Internal
use super::MusicBox;
use crate::music::note::Note;
use crate::prelude::*;

impl MusicBox {
    /// Returns `true` if the note is playable by the musicbox
    pub fn is_valid_note(&self, note: &Note) -> bool {
        self.notes.contains(note)
    }
    /// Returns the vertical distance between two notes
    pub fn vertical_note_distance(&self) -> f64 {
        self.strip_height_mm / (self.notes.len() as f64 - 1f64)
    }

    /// Returns the number of notes
    pub fn note_count(&self) -> usize {
        self.notes.len()
    }

    /// Gets the index of a [super::Note]. 
    pub fn get_index(&self, note: &Note) -> Option<usize> {
        self.notes.iter().position(|el| *el == *note)
    }
}
