// Modules
pub mod functions;

// clap
use clap::ArgMatches;

// midly
use midly::Smf;

// svg
use svg::Document;

// Internal
use crate::{music::music_box::MusicBox, settings::svg::SvgSettings};

#[derive(Debug)]
pub struct MusicBoxConverter {
    args: ArgMatches,
    // Verbose is seperate so we don't have to look it up a lot
    verbose: bool,
    music_box: Option<MusicBox>,
    svg_settings: Option<SvgSettings>,
    svg: Document,
}

impl MusicBoxConverter {
    pub fn new(args: ArgMatches) -> Self {
        let verbose = args.get_one::<bool>("verbosity").unwrap().to_owned();
        Self {
            args,
            verbose,
            music_box: Option::None,
            svg_settings: Option::None,
            svg: Document::new(),
        }
    }
}
