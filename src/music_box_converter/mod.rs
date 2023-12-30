// Modules
pub mod functions;

// clap
use clap::ArgMatches;

// midly
use midly::Smf;

// Internal
use crate::{
    music::{music_box::MusicBox, track::Track},
    settings::settings::{self, Settings},
    svg::document::Document,
    vec2::Vec2,
};

#[derive(Debug, Default)]
pub struct MusicBoxConverter {
    args: ArgMatches,
    // Verbose is seperate so we don't have to look it up a lot
    verbose: bool,
    music_box: Option<MusicBox>,
    svg_settings: Option<Settings>,
    svg: Vec<Document>,
    abs_track: Option<Track>,
    scale: Option<Vec2<f64>>,
}

impl MusicBoxConverter {
    pub fn new(args: ArgMatches) -> Self {
        let verbose = args.get_flag("verbosity").to_owned();
        Self {
            args,
            verbose,
            music_box: Option::None,
            svg_settings: Option::None,
            svg: Vec::<Document>::new(),
            abs_track: Option::None,
            scale: Option::Some(Vec2::<f64>::default()),
        }
    }
}
