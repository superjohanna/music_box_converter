// Modules
pub mod functions;

// clap
use clap::ArgMatches;

// Internal
use crate::music::music_box::MusicBox;

#[derive(Debug)]
pub struct MusicBoxConverter {
    args: ArgMatches,
    music_box: Option<MusicBox>,
}

impl MusicBoxConverter {
    pub fn new(args: ArgMatches) -> Self {
        Self {
            args,
            music_box: Option::None,
        }
    }
}
