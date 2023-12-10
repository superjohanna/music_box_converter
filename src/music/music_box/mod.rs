// Modules
pub mod functions;

use serde_derive::{Deserialize, Serialize};

// Internal
use super::note::Note;

#[derive(Debug, Serialize, Deserialize)]
pub struct MusicBox {
    pub name: String,
    pub strip_height_mm: u32,
    notes: Vec<Note>,
}

impl MusicBox {
    pub fn new(name: String, strip_height_mm: u32, notes: Vec<Note>) -> Self {
        Self {
            name,
            strip_height_mm,
            notes,
        }
    }
}
