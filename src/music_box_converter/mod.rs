// Modules
pub mod functions;

// CLAP
use clap::ArgMatches;

#[derive(Debug)]
pub struct MusicBoxConverter {
    args: ArgMatches,
}

impl MusicBoxConverter {
    pub fn new(args: ArgMatches) -> Self {
        Self { args }
    }
}
