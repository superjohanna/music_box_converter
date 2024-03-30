pub mod command;
pub mod config_macro;
pub mod functions;
pub mod item_list;
pub mod ui;

use std::{default, error::Error, io::Stdout};

// clap
use clap::ArgMatches;

// ratatui
use ratatui::{
    backend::CrosstermBackend,
    widgets::{List, ListState},
    Terminal,
};

// Internal
use self::item_list::settings_item_list::SettingsItemList;
use crate::{lang::LangMap, settings::Settings};

#[derive(Debug, Default)]
pub struct MusicBoxConfig {
    /// The translation
    lang_map: LangMap,
    /// Arguments that are passed through.
    args: ArgMatches,
    /// The Terminal we draw to.
    terminal: Option<Terminal<CrosstermBackend<Stdout>>>,
    /// The current state of our settings. This is what gets serialized and what is deserialized when loading a file to.
    settings: Option<Settings>,
    /// The current state of the item one is editing. This gets put into the respective field of the self.settings.
    input_buf: String,
    /// The stored output path / input path. This will be the file that was passed by arguments or the file that was previously saved.
    path_buf: String,
    /// Index of the item that is currently being edited (The one with the '>>' before it).
    index: usize,
    /// The number of settings + groups there are. We need this to stop the user if they are at the bottom of the list and press down.
    max_index: usize,
    /// The state of the list. This is somehow supposed to allow scrolling? I get to it once I implement scrolling.
    list_state: ListState,
    /// This is a representation of the settings.
    settings_item_list: SettingsItemList,
    /// Indicates wether we have a popup open
    popup: bool,
    /// Indicates wether we had an error while parsing `Self::input_buf`
    parse_error: bool,
    /// Indicates wether we had an error while saving
    save_error: Option<Box<dyn Error>>,
    /// Indicates wether we had an error while opening
    open_error: Option<Box<dyn Error>>,
    /// Indicates wether we are trying to open a file
    open_file: Option<String>,
    /// Indicates wether we are trying to save a file
    save_file: Option<String>,
}

impl MusicBoxConfig {
    pub fn new(args: &ArgMatches) -> Self {
        let list = SettingsItemList::get_items();
        let path = args.get_one::<String>("io_settings").unwrap().clone();
        let locale = match sys_locale::get_locale() {
            Some(t) => t,
            None => "en-GB".to_string(),
        };

        Self {
            lang_map: LangMap::load_from_fs(&("./lang/".to_string() + &locale + ".json")),
            args: args.clone(),
            settings_item_list: SettingsItemList::get_items(),
            input_buf: "Group. Not editable.".to_string(),
            list_state: ListState::default().with_selected(Some(0usize)),
            path_buf: path.clone(),
            open_file: Some(path),
            ..Default::default()
        }
    }
}
