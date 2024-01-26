pub mod config_groups;
pub mod config_macro;
pub mod functions;
pub mod ui;

use std::{default, io::Stdout};

// clap
use clap::ArgMatches;

// ratatui
use ratatui::{
    backend::CrosstermBackend,
    widgets::{List, ListState},
    Terminal,
};

// Internal
use crate::settings::Settings;

use self::config_groups::{get_groups, GroupList, GroupListTrait, ValueType};

#[derive(Debug, Default)]
pub struct MusicBoxConfig {
    /// Arguments that are passed through.
    args: ArgMatches,
    /// The Terminal we draw to.
    terminal: Option<Terminal<CrosstermBackend<Stdout>>>,
    /// The current state of our settings. This is what gets serialized and what is deserialized when loading a file to.
    settings: Option<Settings>,
    /// A ```Vec<SettingsGroup>``` which holds all the fields in their respective groups. Set from [crate::settings] via macros.
    groups: GroupList,
    /// The current state of the item one is editing. This gets put into the respective field of the self.settings.
    input_buf: String,
    /// The stored output path / input path. This will be the file that was passed by arguments or the file that was previously saved.
    output_path: String,
    /// Index of the item that is currently being edited (The one with the '>>' before it).
    settings_index: usize,
    /// The number of settings + groups there are. We need this to stop the user if they are at the bottom of the list and press down.
    settings_arr_length: usize,
    /// The state of the list. This is somehow supposed to allow scrolling? I get to it once I implement scrolling.
    list_state: ListState,
    /// This is a ```Vec<ValueType>``` which is just a representation of the "flattened" settings list.  
    settings_index_value_type: Vec<ValueType>,
    /// Indicates wether we have a popup open
    popup: bool,
    /// Indicates wether we had an error when parsing ```Self::input_buf```
    parse_error: bool,
    /// Indicates wether we are trying to open a file
    open_file: Option<String>,
    /// Indicates wether we are trying to save a file
    save_file: Option<String>,
}

impl MusicBoxConfig {
    pub fn new(args: &ArgMatches) -> Self {
        let groups = get_groups();
        Self {
            args: args.clone(),
            groups: groups.clone(),
            settings_index_value_type: groups.get_list_value_type(),
            input_buf: "Group. Not editable.".to_string(),
            list_state: ListState::default().with_selected(Some(0usize)),
            output_path: args.get_one::<String>("io_settings").unwrap().clone(),
            ..Default::default()
        }
    }
}
