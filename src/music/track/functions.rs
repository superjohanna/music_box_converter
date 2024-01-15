// Internal
use super::Track;
use crate::music::{event::Event, music_box::MusicBox, note::Note};
use crate::prelude::*;

// midly
use midly::{MetaMessage, MidiMessage, Track as MidiTrack};

impl Track {
    pub fn from_midi_track(track: MidiTrack, music_box: &MusicBox, transpose: &bool) -> Self {
        let mut output = Self {
            inner: Vec::<Event>::new(),
            tick_length: u64::MIN,
            min_distance: u64::MAX,
            max_distance: u64::MIN,
        };
        // Current time used for assigning the absolute time value for each Event
        let mut current_time = 0u64;
        // Array used for calculating the min and max distance
        // 127 is the number of midi pitches there are
        let mut last_seen: [Option<u64>; 127] = [Option::None; 127];
        for event in track {
            current_time += u64::from(u32::from(event.delta));

            let (mut pitch, vel) = match event.kind {
                midly::TrackEventKind::Midi {
                    message: MidiMessage::NoteOn { key, vel },
                    ..
                } => (key, vel),
                _ => continue,
            };

            // Musescore doesn't write a NoteOff event but instead writes a NoteOn Event with zero velocity
            // We need to check for that...
            if vel == 0 {
                continue;
            }

            if !transpose {
                if !music_box.is_valid_note(&Note::from_midi_pitch(pitch)) {
                    info!(
                        "Note {0} at {current_time} not playable with music box {1}. Skipping.",
                        Note::from_midi_pitch(pitch),
                        music_box.name
                    );
                    continue;
                }
            } else {
                if !music_box.is_valid_note(&Note::from_midi_pitch(pitch)) {
                    let note = Note::from_midi_pitch(pitch);
                    let note_octave = note.get_octave();
                    for i in -1..=9 {
                        if !music_box.is_valid_note(&note.transpose(i)) {
                            continue;
                        }
                        info!("Transposing note from octave {note_octave} to {i}");
                        pitch = note.transpose(i).to_midi_pitch();
                        break;
                    }
                }
            }

            info!(
                "Found note {} at {current_time}.",
                Note::from_midi_pitch(pitch)
            );

            if last_seen[pitch.as_int() as usize].is_some() {
                let distance = current_time - last_seen[pitch.as_int() as usize].unwrap();
                if distance != 0 {
                    output.min_distance = std::cmp::min(distance, output.min_distance);
                    output.max_distance = std::cmp::max(distance, output.max_distance);
                } else {
                    warn!(
                        "Two notes in your midi are overlapping (having the same time). Ignoring."
                    )
                }
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

impl std::ops::DerefMut for Track {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
