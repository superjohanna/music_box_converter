// Modules
mod command;
pub mod functions;

// clap
use clap::ArgMatches;

// midly
use midly::Smf;

// Internal
use crate::{
    music::{music_box::MusicBox, track::Track},
    settings::{self, Settings},
    svg_writer::document::Document,
    vec2::Vec2,
};

#[derive(Debug, Default)]
pub struct MusicBoxConvert {
    /// The arguments of the program. Need to be passed in with the new method
    args: ArgMatches,
    /// The `MusicBox` that was chosen if any
    music_box: Option<MusicBox>,
    /// The `Settings` that were chosen if any
    settings: Option<Settings>,
    /// A list of svg documents.
    svg: Vec<Document>,
    /// The Track we are currently working on
    track: Option<Track>,
    /// The scale factor
    scale: Option<Vec2<f64>>,
}

impl MusicBoxConvert {
    pub fn new(args: &ArgMatches) -> Self {
        Self {
            args: args.clone(),
            ..Default::default()
        }
    }
}
