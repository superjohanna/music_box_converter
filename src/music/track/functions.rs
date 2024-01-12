// Internal
use super::Track;
use crate::music::{event::Event, note::Note};

// midly
use midly::{MetaMessage, MidiMessage, Track as MidiTrack};

impl Track {
    pub fn from_midi_track(track: MidiTrack) -> Self {
        let mut output = Self {
            inner: Vec::<Event>::new(),
            tick_length: u64::MIN,
            min_distance: u64::MAX,
            max_distance: u64::MIN,
        };
        // Current time used for assigning the absolute time value for each Event
        let mut current_time = 0u64;
        // Array used for calculating the min and max distance
        // 127 ist the number of midi pitches there are
        let mut last_seen: [Option<u64>; 127] = [Option::None; 127];
        for event in track {
            current_time += u64::from(u32::from(event.delta));

            let pitch = match event.kind {
                midly::TrackEventKind::Midi {
                    message: MidiMessage::NoteOn { key, .. },
                    ..
                } => key,
                _ => continue,
            };

            if last_seen[pitch.as_int() as usize].is_some() {
                let distance = current_time - last_seen[pitch.as_int() as usize].unwrap();
                output.min_distance = std::cmp::min(distance, output.min_distance);
                output.max_distance = std::cmp::max(distance, output.max_distance);
            }

            last_seen[u8::from(pitch) as usize] = Some(current_time);

            output
                .inner
                .push(Event::new(Note::from_midi_pitch(pitch), current_time))
        }

        output.tick_length = current_time;
        output
    }

    pub fn tick_length(&self) -> u64 {
        self.tick_length
    }

    pub fn min_distance(&self) -> u64 {
        self.min_distance
    }

    pub fn max_distance(&self) -> u64 {
        self.max_distance
    }
}

impl std::ops::Deref for Track {
    type Target = Vec<Event>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
