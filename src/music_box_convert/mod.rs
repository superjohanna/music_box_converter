// Modules
pub mod functions;

// clap
use clap::ArgMatches;

// midly
use midly::Smf;

// Internal
use crate::{
    music::{music_box::MusicBox, track::Track},
    settings::{self, Settings},
    svg::document::Document,
    vec2::Vec2,
};

#[derive(Debug, Default)]
pub struct MusicBoxConvert {
    args: ArgMatches,
    music_box: Option<MusicBox>,
    settings: Option<Settings>,
    svg: Vec<Document>,
    track: Option<Track>,
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
