// midly
use midly::{num::u7, MidiMessage};

// Internal
use super::Note;

impl Note {
    /// Returns a `Note` from a midi pitch.
    pub fn from_midi_pitch(pitch: midly::num::u7) -> Self {
        let pitch = pitch.as_int();
        let octave = ((pitch - 12) / 12) as i8;
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

    /// Returns a midi pitch from a `Note`
    pub fn to_midi_pitch(&self) -> midly::num::u7 {
        match self {
            Note::C(octave) => u7::from_int_lossy(((octave * 12) + 12) as u8),
            Note::CS(octave) => u7::from_int_lossy((1 + (octave * 12) + 12) as u8),
            Note::D(octave) => u7::from_int_lossy((2 + (octave * 12) + 12) as u8),
            Note::DS(octave) => u7::from_int_lossy((3 + (octave * 12) + 12) as u8),
            Note::E(octave) => u7::from_int_lossy((4 + (octave * 12) + 12) as u8),
            Note::F(octave) => u7::from_int_lossy((5 + (octave * 12) + 12) as u8),
            Note::FS(octave) => u7::from_int_lossy((6 + (octave * 12) + 12) as u8),
            Note::G(octave) => u7::from_int_lossy((7 + (octave * 12) + 12) as u8),
            Note::GS(octave) => u7::from_int_lossy((8 + (octave * 12) + 12) as u8),
            Note::A(octave) => u7::from_int_lossy((9 + (octave * 12) + 12) as u8),
            Note::AS(octave) => u7::from_int_lossy((10 + (octave * 12) + 12) as u8),
            Note::B(octave) => u7::from_int_lossy((11 + (octave * 12) + 12) as u8),
        }
    }

    /// Transposes a `Note` to a given octave
    pub fn transpose(&self, octave: i8) -> Self {
        match self {
            Note::C(_) => Note::C(octave),
            Note::CS(_) => Note::CS(octave),
            Note::D(_) => Note::D(octave),
            Note::DS(_) => Note::DS(octave),
            Note::E(_) => Note::E(octave),
            Note::F(_) => Note::F(octave),
            Note::FS(_) => Note::FS(octave),
            Note::G(_) => Note::G(octave),
            Note::GS(_) => Note::GS(octave),
            Note::A(_) => Note::A(octave),
            Note::AS(_) => Note::AS(octave),
            Note::B(_) => Note::B(octave),
        }
    }

    /// Returns a reference to an octave from a `Note`
    pub fn get_octave(&self) -> &i8 {
        match self {
            Note::C(octave) => octave,
            Note::CS(octave) => octave,
            Note::D(octave) => octave,
            Note::DS(octave) => octave,
            Note::E(octave) => octave,
            Note::F(octave) => octave,
            Note::FS(octave) => octave,
            Note::G(octave) => octave,
            Note::GS(octave) => octave,
            Note::A(octave) => octave,
            Note::AS(octave) => octave,
            Note::B(octave) => octave,
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
