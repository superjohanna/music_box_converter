// Internal
use super::Track;
use crate::music::{event::Event, music_box::MusicBox, note::Note};
use crate::prelude::*;

use midly::num::{u28, u4, u7};
// midly
use midly::{MetaMessage, MidiMessage, Timing, Track as MidiTrack, TrackEvent};

impl Track {
    /// Converts a `MidiTrack` into a `Track`. Removes unplayable notes by the passed `MusicBox` and transposes them by octaves if `transpose` is set
    pub fn from_midi_track(track: MidiTrack, music_box: &MusicBox, transpose: &bool) -> Self {
        let mut output = Self {
            inner: Vec::<Event>::new(),
            tick_length: u64::MIN,
            min_distance: u64::MAX,
            max_distance: u64::MIN,
        };

        info!(
            "Transposing {}",
            if *transpose { "enabled" } else { "disabled" }
        );
        // Current time used for assigning the absolute time value for each Event
        let mut current_time = 0u64;
        // Array used for calculating the min and max distance
        // 127 is the number of midi pitches there are
        let mut last_seen = [Option::None; 127];
        // The outer loop over all `TrackEvents`
        for event in track {
            current_time += u64::from(u32::from(event.delta));

            // Get the pitch and velocity of the event or continue if something else
            let (mut pitch, vel) = match event.kind {
                midly::TrackEventKind::Midi {
                    message: MidiMessage::NoteOn { key, vel },
                    ..
                } => (key, vel),
                // Some midi program send velocity 0 NoteOn events instead of NoteOff events. Just use that here to differentiate between NoteOff and NoteOn
                midly::TrackEventKind::Midi {
                    message: MidiMessage::NoteOff { key, .. },
                    ..
                } => (key, u7::from(0)),
                _ => continue,
            };

            // Continue if it is a note off. Don't waste time calculating stuff for a note that will be discarded
            if vel == 0 {
                continue;
            }

            let note = Note::from_midi_pitch(pitch);

            if !music_box.is_valid_note(&note) {
                // Note can't be played
                match transpose {
                    // No transpose
                    false => {
                        warn!(
                            "Note '{0}' at '{current_time}' with velocity '{vel}' not playable with music box. Skipping.",
                            Note::from_midi_pitch(pitch),
                        );
                        continue;
                    }

                    // Transpose
                    true => {
                        let note_octave = note.get_octave();
                        let mut transposable = false;
                        // This should cover the whole midi pitch spectrum. My converted notes go far beyond this so it doesn't matter if the notes have negative hz or are over 20khz
                        for transpose_octave in -1..=9 {
                            if !music_box.is_valid_note(&note.transpose(transpose_octave)) {
                                continue;
                            }
                            // Could transpose.
                            info!("Transposing note '{note}' at '{current_time}' with velocity '{vel}' from octave '{note_octave}' to '{transpose_octave}'");
                            pitch = note.transpose(transpose_octave).to_midi_pitch();
                            break;
                        }
                        if !transposable {
                            // Couldn't transpose. Continue to next event
                            warn!(
                                "Note '{0}' at '{current_time}' with velocity '{vel}' not playable with music box even when transposing. Skipping.",
                                Note::from_midi_pitch(pitch),
                            );
                            continue;
                        }
                    }
                }
            }

            info!(
                "Found note '{}' at '{current_time}' with velocity '{vel}'.",
                Note::from_midi_pitch(pitch)
            );

            // Distance calculation. If Note hasn't been seen before then ignore
            if last_seen[pitch.as_int() as usize].is_some() {
                let distance = current_time - last_seen[pitch.as_int() as usize].unwrap();
                if distance != 0 {
                    output.min_distance = std::cmp::min(distance, output.min_distance);
                    output.max_distance = std::cmp::max(distance, output.max_distance);
                } else {
                    info!(
                        "Two notes '{0}' are overlapping at '{current_time}'. Ignoring for distance calculation.",
                        Note::from_midi_pitch(pitch),
                    )
                }
            }

            // Saving that a note has been encountered
            if vel != 0 {
                last_seen[u8::from(pitch) as usize] = Some(current_time);
            }

            // Add to track
            output.inner.push(Event::new(
                Note::from_midi_pitch(pitch),
                current_time,
                vel.as_int(),
            ))
        }

        // This is the total length in MidiTicks
        output.tick_length = current_time;
        output
    }

    /// Converts a `Track` into a `MidiTrack`. Copies the Midi meta events from the passed track.
    pub fn to_midi_track<'a>(&self, track: MidiTrack<'a>) -> MidiTrack<'a> {
        // Create new track
        let mut output = MidiTrack::default();

        // Name it
        output.push(TrackEvent {
            delta: u28::from(0),
            kind: midly::TrackEventKind::Meta(midly::MetaMessage::TrackName(
                "music_box_converter".as_bytes(),
            )),
        });

        // Copy the time signature if there is one
        match track.iter().find(|t| {
            matches!(
                t.kind,
                midly::TrackEventKind::Meta(midly::MetaMessage::TimeSignature(..))
            )
        }) {
            None => (),
            Some(t) => output.push(*t),
        }

        // Copy the key signature if there is one
        match track.iter().find(|t| {
            matches!(
                t.kind,
                midly::TrackEventKind::Meta(midly::MetaMessage::KeySignature(..))
            )
        }) {
            None => (),
            Some(t) => output.push(*t),
        }

        // Copy the Tempo if there is one
        match track.iter().find(|t| {
            matches!(
                t.kind,
                midly::TrackEventKind::Meta(midly::MetaMessage::Tempo(..))
            )
        }) {
            None => (),
            Some(t) => output.push(*t),
        }

        // Convert from absolute to delta and push it onto the track
        let mut prev_abs = 0;
        for event in self.inner.clone() {
            output.push(TrackEvent {
                delta: u28::from((event.abs - prev_abs) as u32),
                kind: midly::TrackEventKind::Midi {
                    channel: u4::from(0),
                    message: MidiMessage::NoteOn {
                        key: event.note.to_midi_pitch(),
                        vel: u7::from(event.vel),
                    },
                },
            });
            prev_abs = event.abs;
        }

        // Add an EndOfTrack Message
        output.push(TrackEvent {
            delta: u28::from(0),
            kind: midly::TrackEventKind::Meta(MetaMessage::EndOfTrack),
        });

        output
    }

    /// The length in MidiTicks
    pub fn tick_length(&self) -> u64 {
        self.tick_length
    }

    /// The minimum distance between two notes of the same key.
    pub fn min_distance(&self) -> u64 {
        self.min_distance
    }

    /// The maximum distance between two notes of the same key.
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
