use super::Event;
use crate::music::note::Note;

impl Event {
    pub fn new(note: Note, abs: u64) -> Self {
        Self { note, abs }
    }
}
