// std
use std::{
    fs::File,
    io::{stdout, BufReader, Stdout, Write},
};

// crossterm
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

// ratatui
use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::Paragraph,
};
use serde::{Serialize, Serializer};

// Internal
use super::{
    item_list::value::{ValueType, ValueWrapper},
    ui::ui,
    MusicBoxConfig,
};
use crate::{prelude::*, settings::Settings};

impl MusicBoxConfig {
    pub fn run(&mut self) -> Result<()> {
        stdout().execute(EnterAlternateScreen).to_res()?;
        enable_raw_mode().to_res()?;
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout())).to_res()?;
        terminal.clear().to_res()?;

        if let Err(e) = self.open() {
            self.open_error = Some(Box::new(e));
            self.open_file = None;
            self.popup = true;
        }
        self.main_loop(&mut terminal);

        stdout().execute(LeaveAlternateScreen).to_res()?;
        disable_raw_mode().to_res()?;
        Ok(())
    }

    fn main_loop(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
        loop {
            terminal
                .draw(|frame| {
                    ui(frame, self);
                })
                .to_res()?;

            // I'm sorry about the level of indentation. I'll refactor it later. Hopefully if let chaining becomes stable soon
            if event::poll(std::time::Duration::from_millis(100)).to_res()? {
                if let Event::Key(key) = event::read().to_res()? {
                    if !(key.kind == KeyEventKind::Press) {
                        continue;
                    }
                    if key.modifiers == KeyModifiers::CONTROL {
                        match key.code {
                            KeyCode::Char('x') => {
                                if !self.popup {
                                    break;
                                }
                                self.save_file = None;
                                self.open_file = None;
                                self.parse_error = false;
                                if self.open_error.is_some() {
                                    self.settings = Some(Settings::default());
                                }
                                self.open_error = None;
                                self.save_error = None;
                                self.popup = false;
                            }
                            KeyCode::Char('l') => {
                                if self.popup {
                                    continue;
                                }
                                if self.settings_item_list[self.index].value_type
                                    == ValueType::Boolean
                                {
                                    self.input_buf = "false".to_string();
                                    continue;
                                }
                                self.input_buf = String::new();
                            }
                            KeyCode::Char('s') => {
                                if self.popup {
                                    continue;
                                }
                                self.save_file = Some(self.path_buf.clone());
                                self.popup = true;
                            }
                            KeyCode::Char('o') => {
                                if self.popup {
                                    continue;
                                }
                                self.open_file = Some(self.path_buf.clone());
                                self.popup = true;
                            }
                            KeyCode::Char('e') => {
                                if self.index != 0 {
                                    if self.update_settings_index(true).is_err() {
                                        continue;
                                    }
                                    self.index -= 1;
                                }
                            }
                            KeyCode::Char('d') => {
                                if self.index != self.max_index {
                                    if self.update_settings_index(false).is_err() {
                                        continue;
                                    }
                                    self.index += 1;
                                }
                            }
                            _ => (),
                        }
                    } else if key.modifiers == KeyModifiers::SHIFT {
                        match key.code {
                            KeyCode::Char(c) => {
                                if self.popup {
                                    continue;
                                }
                                if self.settings_item_list[self.index].value_type
                                    == ValueType::Boolean
                                {
                                    if c == ' ' {
                                        if &self.input_buf == "true" {
                                            self.input_buf = "false".to_string()
                                        } else {
                                            self.input_buf = "true".to_string()
                                        }
                                    }
                                    continue;
                                }
                                if let Some(t) = &mut self.save_file {
                                    t.push(c);
                                } else if let Some(t) = &mut self.open_file {
                                    t.push(c);
                                } else {
                                    self.input_buf += &c.to_uppercase().collect::<String>();
                                }
                            }
                            KeyCode::Backspace => {
                                if self.popup {
                                    continue;
                                }
                                if let Some(t) = &mut self.save_file {
                                    t.pop().res();
                                } else if let Some(t) = &mut self.open_file {
                                    t.pop();
                                } else {
                                    self.input_buf.pop();
                                }
                            }
                            _ => continue,
                        }
                    } else if key.modifiers == KeyModifiers::NONE {
                        match key.code {
                            KeyCode::Char(c) => {
                                if self.popup {
                                    continue;
                                }
                                if self.settings_item_list[self.index].value_type
                                    == ValueType::Boolean
                                {
                                    if c == ' ' {
                                        if &self.input_buf == "true" {
                                            self.input_buf = "false".to_string()
                                        } else {
                                            self.input_buf = "true".to_string()
                                        }
                                    }
                                    continue;
                                }
                                if let Some(t) = &mut self.save_file {
                                    t.push(c);
                                } else if let Some(t) = &mut self.open_file {
                                    t.push(c);
                                } else {
                                    self.input_buf.push(c);
                                }
                            }
                            KeyCode::Backspace => {
                                if self.parse_error {
                                    continue;
                                }
                                if let Some(t) = &mut self.save_file {
                                    t.pop();
                                } else if let Some(t) = &mut self.open_file {
                                    t.pop();
                                } else {
                                    self.input_buf.pop();
                                }
                            }
                            KeyCode::Enter => {
                                if !self.popup {
                                    continue;
                                }
                                if self.parse_error {
                                    self.parse_error = false;
                                }
                                if self.save_error.is_some() {
                                    self.save_error = None;
                                }
                                if self.open_error.is_some() {
                                    self.settings = Some(Settings::default());
                                    self.open_error = None;
                                }
                                if let Some(t) = &self.save_file {
                                    self.path_buf = t.clone();
                                    self.save_current_setting();
                                    if let Err(e) = self.save() {
                                        self.save_error = Some(Box::new(e));
                                        self.save_file = None;
                                        continue;
                                    };
                                }
                                if let Some(t) = &self.open_file {
                                    self.path_buf = t.clone();
                                    if let Err(e) = self.open() {
                                        self.open_error = Some(Box::new(e));
                                        self.open_file = None;
                                        continue;
                                    };
                                    self.load_current_setting();
                                }
                                self.popup = false;
                            }
                            KeyCode::Up => {
                                if self.index != 0 {
                                    if self.update_settings_index(true).is_err() {
                                        continue;
                                    }
                                    self.index -= 1;
                                    continue;
                                }
                            }
                            KeyCode::Down => {
                                if self.index != self.max_index {
                                    if self.update_settings_index(false).is_err() {
                                        continue;
                                    }
                                    self.index += 1;
                                    continue;
                                }
                            }
                            KeyCode::Esc => {
                                self.parse_error = false;
                                self.save_file = None;
                                self.open_file = None;
                                if self.open_error.is_some() {
                                    self.settings = Some(Settings::default());
                                }
                                self.open_error = None;
                                self.save_error = None;
                                self.popup = false;
                            }
                            _ => continue,
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// For updating the settings_index in a direction. Loads stuff into the input_buffer for example.
    fn update_settings_index(&mut self, negative: bool) -> Result<()> {
        // Check if popup
        if self.popup {
            return Err(Error::Generic("Popup".to_string()));
        }
        // Load next value
        let (prev_value_type, prev_index) =
            (self.settings_item_list[self.index].value_type, self.index);
        let (next_value_type, next_index) = match negative {
            true => (
                self.settings_item_list[self.index - 1].value_type,
                prev_index - 1,
            ),
            false => (
                self.settings_item_list[self.index + 1].value_type,
                prev_index + 1,
            ),
        };

        let prev_wrapper: Option<ValueWrapper> = match prev_value_type {
            ValueType::None => None,
            ValueType::Colour => Some(ValueWrapper::String(self.input_buf.clone())),
            ValueType::Number => match self.input_buf.parse() {
                Ok(t) => Some(ValueWrapper::F64(t)),
                Err(_) => {
                    self.parse_error = true;
                    self.popup = true;
                    return Err(Error::Generic("Couldn't parse".to_string()));
                }
            },
            ValueType::Boolean => match self.input_buf.parse() {
                Ok(t) => Some(ValueWrapper::Boolean(t)),
                Err(_) => {
                    panic!("buffer wasn't true or false");
                }
            },
        };

        if let Some(t) = prev_wrapper {
            self.settings.res_mut()?.set(prev_index, &t);
        }

        let next_wrapper = self.settings.res()?.get(next_index);

        if let Some(t) = next_wrapper {
            self.input_buf = t.to_string();
            return Ok(());
        }
        self.input_buf = self.lang_map.val_at("capital.groupBuffer.fullStop");
        Ok(())
    }

    /// Reduced update_settings_index
    fn save_current_setting(&mut self) -> Result<()> {
        let (value_type, index) = (self.settings_item_list[self.index].value_type, self.index);

        let wrapper: Option<ValueWrapper> = match value_type {
            ValueType::None => None,
            ValueType::Colour => Some(ValueWrapper::String(self.input_buf.clone())),
            ValueType::Number => match self.input_buf.parse() {
                Ok(t) => Some(ValueWrapper::F64(t)),
                Err(_) => {
                    self.parse_error = true;
                    self.popup = true;
                    return Err(Error::Generic("Couldn't parse".to_string()));
                }
            },
            ValueType::Boolean => match self.input_buf.parse() {
                Ok(t) => Some(ValueWrapper::Boolean(t)),
                Err(_) => {
                    panic!("buffer wasn't true or false");
                }
            },
        };

        if let Some(t) = wrapper {
            self.settings.res_mut()?.set(index, &t);
        }

        Ok(())
    }

    /// Reduced update_settings_index
    fn load_current_setting(&mut self) -> Result<()> {
        let value_type = self.settings_item_list[self.index].value_type;
        let wrapper = self.settings.res()?.get(self.index);

        if let Some(t) = wrapper {
            self.input_buf = t.to_string();
            return Ok(());
        }
        Ok(())
    }

    /// Opens a file with the path provided by self.open_file and deserializes it to self.settings.
    fn open(&mut self) -> Result<()> {
        let path_string = self.open_file.clone().unwrap();
        let abs_path = match crate::path::absolute_path(path_string.clone()) {
            Ok(t) => t,
            Err(e) => return Err(Error::IOError(Box::new(e), Box::new(path_string))),
        };

        let file = match File::open(abs_path) {
            Ok(t) => t,
            Err(e) => return Err(Error::IOError(Box::new(e), Box::new(path_string))),
        };

        let deserialized: Settings = match serde_json::from_reader(BufReader::new(file)) {
            Ok(t) => t,
            Err(e) => return Err(Error::SerdeJsonError(Box::new(e))),
        };

        self.settings = Some(deserialized);

        self.open_file = None;

        Ok(())
    }

    /// Saves a file to the path provided by self.save_file and serializes self.settings to it.
    fn save(&mut self) -> Result<()> {
        let mut path_string = self.path_buf.clone();
        let mut abs_path = match crate::path::absolute_path(path_string.clone()) {
            Ok(t) => t,
            Err(e) => return Err(Error::IOError(Box::new(e), Box::new(path_string))),
        };

        let parent = abs_path.parent();

        if let Some(t) = parent {
            match std::fs::create_dir_all(t) {
                Ok(t) => t,
                Err(e) => (), /*return Err(Error::IOError(Box::new(e)))*/
            }
        }

        let mut file = std::fs::File::create(abs_path).to_res()?;

        let j = match serde_json::to_string_pretty(self.settings.res()?) {
            Ok(t) => t,
            Err(e) => return Err(Error::SerdeJsonError(Box::new(e))),
        };

        file.write_all(j.as_bytes());

        self.save_file = None;

        Ok(())
    }
}
