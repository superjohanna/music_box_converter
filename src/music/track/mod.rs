pub mod functions;

// Internal
use super::event::Event;

// midly
use midly::Track as MidiTrack;

#[derive(Debug, Clone, std::default::Default)]
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
