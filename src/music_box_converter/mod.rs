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
pub struct MusicBoxConverter {
    args: ArgMatches,
    music_box: Option<MusicBox>,
    svg_settings: Option<Settings>,
    svg: Vec<Document>,
    track: Option<Track>,
    scale: Option<Vec2<f64>>,
}

impl MusicBoxConverter {
    pub fn new(args: ArgMatches) -> Self {
        Self {
            args,
            music_box: Option::None,
            svg_settings: Option::None,
            svg: Vec::<Document>::new(),
            track: Option::None,
            scale: Option::Some(Vec2::<f64>::default()),
        }
    }
}
