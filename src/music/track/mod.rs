pub mod functions;

// Internal
use super::event::Event;

// midly
use midly::{Timing, Track as MidiTrack};

// std
use std::default::Default;

/// My representation of a midi track. Contains metadata for calculations.
#[derive(Debug, Clone, Default)]
pub struct Track {
    inner: Vec<Event>,
    tick_length: u64,
    min_distance: u64,
    max_distance: u64,
}

impl Track {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
