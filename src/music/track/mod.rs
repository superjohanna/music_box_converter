pub mod functions;

// Internal
use super::event::Event;

// midly
use midly::Track as MidiTrack;

#[derive(Debug, Clone)]
pub struct Track {
    inner: Vec<Event>,
    tick_length: u64,
    min_distance: u64,
    max_distance: u64,
}
