// midly
use midly::MidiMessage;

// Internal
use super::Note;

impl Note {
    pub fn from_midi_message(message: MidiMessage) -> Option<Self> {
        match message {
            MidiMessage::NoteOn { key, .. } => return Self::from_midi_pitch(key),
            _ => return None,
        }
    }

    pub fn from_midi_pitch(p: midly::num::u7) -> Option<Self> {
        let pitch = p.as_int();
        if pitch < 21 {
            return None;
        }
        let octave = ((pitch - 21) / 12);
        match pitch % 12 {
            0 => Some(Note::C(octave)),
            1 => Some(Note::CS(octave)),
            2 => Some(Note::D(octave)),
            3 => Some(Note::DS(octave)),
            4 => Some(Note::E(octave)),
            5 => Some(Note::F(octave)),
            6 => Some(Note::FS(octave)),
            7 => Some(Note::G(octave)),
            8 => Some(Note::GS(octave)),
            9 => Some(Note::A(octave)),
            10 => Some(Note::AS(octave)),
            11 => Some(Note::B(octave)),
            _ => panic!("/music/note/function.rs/from_midi_pitch(). Impossible value!"),
        }
    }
}
