// Modules
pub mod functions;

// serde_derive
use serde_derive::Deserialize;

// Internal
use super::note::Note;

/// A music box
#[derive(Debug, Deserialize)]
pub struct MusicBox {
    /// The strip height in millimetres. The strip is the length of all note lines plus the border
    pub strip_height_mm: f64,
    /// The minimum distance the music box can play two notes back to back. We need this to calculate how much we need to stretch the notes for all of them to playable
    pub min_note_distance_mm: f64,
    /// The Notes this music box can play
    notes: Vec<Note>,
}

impl MusicBox {
    pub fn new(strip_height_mm: f64, min_note_distance_mm: f64, notes: Vec<Note>) -> Self {
        Self {
            strip_height_mm,
            min_note_distance_mm,
            notes,
        }
    }
}
