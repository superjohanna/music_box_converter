pub mod functions;

use super::note::Note;

#[derive(Debug, Clone)]
pub struct Event {
    /// The Note of the Event
    pub note: Note,
    /// The absolute time of the <code>Event</code>
    pub abs: u64,
}
