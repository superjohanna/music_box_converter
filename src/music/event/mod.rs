pub mod functions;

use super::note::Note;

/// My representation of a midi event. This is absolute instead of just being the delta.
#[derive(Debug, Clone)]
pub struct Event {
    /// The Note of the Event
    pub note: Note,
    /// The absolute time of the <code>Event</code>
    pub abs: u64,
    /// The velocity of the Event
    pub vel: u8,
}
