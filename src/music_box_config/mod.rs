pub mod area;
pub mod command;
pub mod config_macro;
pub mod functions;
pub mod item_list;
pub mod key_handler;
pub mod state;
pub mod ui;

use std::{default, io::Stdout, ops::Deref};

// clap
use clap::ArgMatches;

// ratatui
use ratatui::{
    backend::CrosstermBackend,
    widgets::{List, ListState},
    Terminal,
};

// Internal
use self::{area::Areas, key_handler::KeyPressEvent};
use self::{item_list::settings_item_list::SettingsItemList, state::ApplicationState};
use crate::prelude::*;
use crate::{lang::LangMap, settings::Settings};

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub enum ExlusiveBuffers {
    /// We have no exlusive buffers
    #[default]
    None,
    /// We have an open file popup and so we need a buffer for it
    OpenFile(String),
    /// We have a save file popup and so we need a buffer for it
    SaveFile(String),
}

impl ExlusiveBuffers {
    pub fn as_ref(&self) -> Option<&String> {
        match self {
            Self::None => None,
            Self::OpenFile(s) => Some(s),
            Self::SaveFile(s) => Some(s),
        }
    }

    pub fn as_ref_mut(&mut self) -> Option<&mut String> {
        match self {
            Self::None => None,
            Self::OpenFile(s) => Some(s),
            Self::SaveFile(s) => Some(s),
        }
    }
}

/// Holds various buffers
#[derive(Debug)]
pub struct Buffers {
    /// The current state of the item one is editing. This gets put into the respective field of the [MusicBoxConfig::settings] upon changing item
    pub editor_buffer: String,
    /// The stored output / input path. This will be the file that was passed by arguments or the file that was previously saved / opened
    pub path_buffer: String,
    /// Holds various varients of exlusive buffers e.g. open file, save file buffer or none
    pub exlusive_buffer: ExlusiveBuffers,
    /// Holds an error for a variety of popups. This has an uninitialized error until a real error occurs.
    pub error_buffer: Box<dyn std::error::Error>,
}

impl Default for Buffers {
    fn default() -> Self {
        Self {
            editor_buffer: Default::default(),
            path_buffer: Default::default(),
            exlusive_buffer: Default::default(),
            error_buffer: Box::new(Error::Generic(
                "Uninitialized error, report to author".to_string(),
            )),
        }
    }
}

#[derive(Debug, Default)]
pub struct MusicBoxConfig {
    /// Denotes the change that should be happening to the state
    key_press_event: KeyPressEvent,
    /// The current state of the application e.g. what to render
    state: ApplicationState,
    /// Various buffers see [Buffers]
    buffers: Buffers,
    /// The Rects that split up the area of the terminal
    area: Option<Areas>,

    /// The translation map
    lang_map: LangMap,
    /// Arguments that are passed through.
    args: ArgMatches,
    /// The Terminal we draw to.
    terminal: Option<Terminal<CrosstermBackend<Stdout>>>,
    /// The current state of our settings. This is what gets serialized and what is deserialized when loading a file to.
    settings: Settings,
    /// Index of the item that is currently being edited (The one with the '>>' before it).
    index: usize,
    /// The number of settings + groups there are. We need this to stop the user if they are at the bottom of the list and press down.
    max_index: usize,
    /// The state of the list. This is somehow supposed to allow scrolling? I get to it once I implement scrolling.
    list_state: ListState,
    /// This is a representation of the settings.
    settings_item_list: SettingsItemList,
}

impl MusicBoxConfig {
    pub fn new(args: &ArgMatches) -> Self {
        let list = SettingsItemList::get_items();
        let path_buf = args.get_one::<String>("io_settings").unwrap().clone();
        let settings_item_list = SettingsItemList::get_items();
        let locale = match sys_locale::get_locale() {
            Some(t) => t,
            None => "en-GB".to_string(),
        };
        let lang_map =
            LangMap::load_from_fs(&("./lang/".to_string() + &locale + ".json")).unwrap_or_default();

        Self {
            args: args.clone(),
            max_index: settings_item_list.len() - 1,
            settings_item_list,
            list_state: ListState::default().with_selected(Some(0usize)),
            buffers: Buffers {
                path_buffer: path_buf,
                editor_buffer: lang_map.val_at("capital.groupBuffer.fullStop").to_owned(),
                ..Default::default()
            },
            lang_map,
            ..Default::default()
        }
    }
}
