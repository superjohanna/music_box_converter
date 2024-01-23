pub mod functions;
pub mod ui;

use std::{default, io::Stdout};

// clap
use clap::ArgMatches;

// ratatui
use ratatui::{backend::CrosstermBackend, widgets::ListState, Terminal};

// Internal
use crate::settings::Settings;

#[derive(Debug, Default)]
pub struct MusicBoxConfig {
    args: ArgMatches,
    terminal: Option<Terminal<CrosstermBackend<Stdout>>>,
    settings: Option<Settings>,
    value_input: String,
    output_path: String,
    current_setting: usize,
    current_setting_max_length: usize,
    list_state: ListState,
}

impl MusicBoxConfig {
    pub fn new(args: &ArgMatches) -> Self {
        Self {
            args: args.clone(),
            ..Default::default()
        }
    }
}
