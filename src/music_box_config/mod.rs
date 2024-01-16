pub mod functions;

use std::{default, io::Stdout};

// clap
use clap::ArgMatches;

// ratatui
use ratatui::{backend::CrosstermBackend, Terminal};

// Internal
use crate::settings::Settings;

#[derive(Debug, Default)]
pub struct MusicBoxConfig {
    args: ArgMatches,
    terminal: Option<Terminal<CrosstermBackend<Stdout>>>,
    settings: Option<Settings>,
    value_input: String,
    output_path: String,
    current_action: CurrentAction,
    current_section: CurrentSection,
}

impl MusicBoxConfig {
    pub fn new(args: &ArgMatches) -> Self {
        Self {
            args: args.clone(),
            ..Default::default()
        }
    }
}

#[derive(Debug, Default)]
pub enum CurrentAction {
    #[default]
    Viewing,
    Editing,
}

#[derive(Debug, Default)]
pub enum CurrentSection {
    #[default]
    Overview,
    Holes,
    Staff(Staff),
}

#[derive(Debug)]
pub enum Staff {
    Lines,
    BoundingBox,
}
