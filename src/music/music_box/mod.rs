// Modules
pub mod functions;

// serde_derive
use serde_derive::Deserialize;

// Internal
use super::note::Note;

#[derive(Debug, Deserialize)]
pub struct MusicBox {
    pub name: String,
    pub strip_height_mm: f32,
    notes: Vec<Note>,
}

impl MusicBox {
    pub fn new(name: String, strip_height_mm: f32, notes: Vec<Note>) -> Self {
        Self {
            name,
            strip_height_mm,
            notes,
        }
    }
}
