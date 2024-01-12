// midly
use midly::MidiMessage;

// Internal
use super::Note;

impl Note {
    pub fn from_midi_pitch(p: midly::num::u7) -> Self {
        let pitch = p.as_int();
        let octave = ((pitch - 21) / 12) as i8;
        match pitch % 12 {
            0 => Note::C(octave),
            1 => Note::CS(octave),
            2 => Note::D(octave),
            3 => Note::DS(octave),
            4 => Note::E(octave),
            5 => Note::F(octave),
            6 => Note::FS(octave),
            7 => Note::G(octave),
            8 => Note::GS(octave),
            9 => Note::A(octave),
            10 => Note::AS(octave),
            11 => Note::B(octave),
            _ => panic!("/music/note/function.rs/from_midi_pitch(). Impossible value!"),
        }
    }
}

impl std::fmt::Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Note::C(o) => format!("C{}", o),
                Note::CS(o) => format!("C♯{}", o),
                Note::D(o) => format!("D{}", o),
                Note::DS(o) => format!("D♯{}", o),
                Note::E(o) => format!("E{}", o),
                Note::F(o) => format!("F{}", o),
                Note::FS(o) => format!("F♯{}", o),
                Note::G(o) => format!("G{}", o),
                Note::GS(o) => format!("G♯{}", o),
                Note::A(o) => format!("A{}", o),
                Note::AS(o) => format!("A♯{}", o),
                Note::B(o) => format!("B{}", o),
            }
        )
    }
}
