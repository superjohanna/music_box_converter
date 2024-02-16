use super::Event;
use crate::music::note::Note;

impl Event {
    pub fn new(note: Note, abs: u64, vel: u8) -> Self {
        Self { note, abs, vel }
    }
}
