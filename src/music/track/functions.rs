// Internal
use super::Track;
use crate::music::{event::Event, music_box::MusicBox, note::Note};
use crate::prelude::*;

use midly::num::{u28, u4, u7};
// midly
use midly::{MetaMessage, MidiMessage, Track as MidiTrack, TrackEvent};

impl Track {
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
        'i: for event in track {
            current_time += u64::from(u32::from(event.delta));

            let (mut pitch, vel) = match event.kind {
                midly::TrackEventKind::Midi {
                    message: MidiMessage::NoteOn { key, vel },
                    ..
                } => (key, vel),
                midly::TrackEventKind::Midi {
                    message: MidiMessage::NoteOff { key, .. },
                    ..
                } => (key, u7::from(0)),
                _ => continue,
            };

            if !transpose {
                // Not transpose. Just check if it's playable
                if !music_box.is_valid_note(&Note::from_midi_pitch(pitch)) {
                    // Not playable
                    warn!(
                        "Note '{0}' at '{current_time}' with velocity '{vel}' not playable with music box. Skipping.",
                        Note::from_midi_pitch(pitch),
                    );
                    continue;
                }
            } else if !music_box.is_valid_note(&Note::from_midi_pitch(pitch)) {
                // Transpose and unplayable note
                let note = Note::from_midi_pitch(pitch);
                let note_octave = note.get_octave();
                // This should cover the whole midi pitch spectrum. My converted notes go far beyond this so it doesn't matter if the notes have negative hz or are over 20khz
                'j: for i in -1..=9 {
                    if !music_box.is_valid_note(&note.transpose(i)) {
                        continue;
                    }
                    info!("Transposing note '{note}' at '{current_time}' with velocity '{vel}' from octave '{note_octave}' to '{i}'");
                    pitch = note.transpose(i).to_midi_pitch();
                    continue 'i;
                }
                // Couldn't transpose
                warn!(
                    "Note '{0}' at '{current_time}' with velocity '{vel}' not playable with music box even when transposing. Skipping.",
                    Note::from_midi_pitch(pitch),
                );
                continue;
            }

            info!(
                "Found note '{}' at '{current_time}' with velocity '{vel}'.",
                Note::from_midi_pitch(pitch)
            );

            if vel != 0 && last_seen[pitch.as_int() as usize].is_some() {
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

            if vel != 0 {
                last_seen[u8::from(pitch) as usize] = Some(current_time);
            }

            output.inner.push(Event::new(
                Note::from_midi_pitch(pitch),
                current_time,
                vel.as_int(),
            ))
        }

        output.tick_length = current_time;
        output
    }

    pub fn to_midi_track<'a>(&self, track: MidiTrack<'a>) -> MidiTrack<'a> {
        /* let filtered: Vec<TrackEvent<'_>> = track
        .clone()
        .into_iter()
        .filter(|t| matches!(t.kind, midly::TrackEventKind::Meta(..)))
        .collect(); */

        let mut output = MidiTrack::default();
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

        let mut prev_abs = 0;
        for event in self.inner.clone() {
            let te = TrackEvent {
                delta: u28::from((event.abs - prev_abs) as u32),
                kind: midly::TrackEventKind::Midi {
                    channel: u4::from(0),
                    message: MidiMessage::NoteOn {
                        key: event.note.to_midi_pitch(),
                        vel: u7::from(event.vel),
                    },
                },
            };
            output.push(te);
            prev_abs = event.abs;
        }

        output.push(TrackEvent {
            delta: u28::from(0),
            kind: midly::TrackEventKind::Meta(MetaMessage::EndOfTrack),
        });

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
