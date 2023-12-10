// Internal
use super::MusicBox;
use crate::music::note::Note;
use crate::prelude::*;

impl MusicBox {
    pub fn is_valid_note(&self, note: &Note) -> bool {
        self.notes.contains(note)
    }
}
