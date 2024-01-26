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
use super::{config_groups::ValueType, ui::ui, MusicBoxConfig};
use crate::{
    prelude::*,
    settings::{Settings, StringOrF64},
};

impl MusicBoxConfig {
    pub fn run(&mut self) -> Result<()> {
        stdout().execute(EnterAlternateScreen).to_res()?;
        enable_raw_mode().to_res()?;
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout())).to_res()?;
        terminal.clear().to_res()?;

        self.settings = Some(Settings::default());
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
                            KeyCode::Char('x') => break,
                            KeyCode::Char('l') => {
                                if self.popup {
                                    continue;
                                }
                                self.input_buf = String::new()
                            }
                            KeyCode::Char('s') => {
                                if self.popup {
                                    continue;
                                }
                                self.save_file = Some(self.output_path.clone());
                                self.popup = true;
                            }
                            KeyCode::Char('o') => {
                                if self.popup {
                                    continue;
                                }
                                self.open_file = Some(self.output_path.clone());
                                self.popup = true;
                            }
                            _ => (),
                        }
                    } else if key.modifiers == KeyModifiers::SHIFT {
                        match key.code {
                            KeyCode::Char(c) => {
                                if self.popup {
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
                                if self.parse_error {
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
                                if let Some(t) = &self.save_file {
                                    self.output_path = t.clone();
                                    self.save_current_setting();
                                    self.save();
                                    self.save_file = None;
                                }
                                if let Some(t) = &self.open_file {
                                    self.output_path = t.clone();
                                    self.open();
                                    self.load_current_setting();
                                    self.open_file = None;
                                }
                                self.popup = false;
                            }
                            KeyCode::Up => {
                                if self.settings_index != 0 {
                                    if self.update_settings_index(true).is_err() {
                                        continue;
                                    }
                                    self.settings_index -= 1;
                                }
                            }
                            KeyCode::Down => {
                                if self.settings_index != self.settings_arr_length {
                                    if self.update_settings_index(false).is_err() {
                                        continue;
                                    }
                                    self.settings_index += 1;
                                }
                            }
                            KeyCode::Esc => {
                                self.parse_error = false;
                                self.save_file = None;
                                self.open_file = None;
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

    fn update_settings_index(&mut self, negative: bool) -> Result<()> {
        // Load next value
        let (prev_value_type, prev_index) = (
            self.settings_index_value_type[self.settings_index],
            self.settings_index,
        );
        let (next_value_type, next_index) = match negative {
            true => (
                self.settings_index_value_type[self.settings_index - 1],
                prev_index - 1,
            ),
            false => (
                self.settings_index_value_type[self.settings_index + 1],
                prev_index + 1,
            ),
        };

        let prev_op: Option<StringOrF64> = match prev_value_type {
            ValueType::None => None,
            ValueType::Colour => Some(StringOrF64::String(self.input_buf.clone())),
            ValueType::Number => {
                let res = self.input_buf.parse();
                match res {
                    Ok(t) => Some(StringOrF64::F64(t)),
                    Err(_) => {
                        self.parse_error = true;
                        self.popup = true;
                        return Err(Error::Generic("Couldn't parse".to_string()));
                    }
                }
            }
        };

        if let Some(t) = prev_op {
            self.settings.res_mut()?.set(prev_index, &t);
        }

        let next_op = self.settings.res()?.get(next_index);

        if let Some(t) = next_op {
            self.input_buf = t.to_string();
            return Ok(());
        }
        self.input_buf = "Group. Not editable".to_string();
        Ok(())
    }

    fn save_current_setting(&mut self) -> Result<()> {
        let (prev_value_type, prev_index) = (
            self.settings_index_value_type[self.settings_index],
            self.settings_index,
        );

        let prev_op: Option<StringOrF64> = match prev_value_type {
            ValueType::None => None,
            ValueType::Colour => Some(StringOrF64::String(self.input_buf.clone())),
            ValueType::Number => {
                let res = self.input_buf.parse();
                match res {
                    Ok(t) => Some(StringOrF64::F64(t)),
                    Err(_) => {
                        self.parse_error = true;
                        self.popup = true;
                        return Err(Error::Generic("Couldn't parse".to_string()));
                    }
                }
            }
        };

        if let Some(t) = prev_op {
            self.settings.res_mut()?.set(prev_index, &t);
        }

        Ok(())
    }

    fn load_current_setting(&mut self) -> Result<()> {
        let next_value_type = self.settings_index_value_type[self.settings_index];
        let next_op = self.settings.res()?.get(self.settings_index);

        if let Some(t) = next_op {
            self.input_buf = t.to_string();
            return Ok(());
        }
        Ok(())
    }

    /// Deserializes ./svg_settings.json and assigns the deserialized SvgSettings to self.svg_settings.
    fn open(&mut self) -> Result<()> {
        let path_string = self.open_file.clone().unwrap();
        let abs_path = match crate::path::absolute_path(path_string) {
            Ok(t) => t,
            Err(e) => return Err(Error::IOError(Box::new(e))),
        };

        let file = match File::open(abs_path) {
            Ok(t) => t,
            Err(e) => return Err(Error::IOError(Box::new(e))),
        };

        let deserialized: Settings = match serde_json::from_reader(BufReader::new(file)) {
            Ok(t) => t,
            Err(e) => return Err(Error::SerdeJsonError(Box::new(e))),
        };

        self.settings = Some(deserialized);
        Ok(())
    }

    fn save(&self) -> Result<()> {
        let mut path_string = self.output_path.clone();
        let mut abs_path = match crate::path::absolute_path(path_string) {
            Ok(t) => t,
            Err(e) => return Err(Error::IOError(Box::new(e))),
        };

        match std::fs::create_dir_all(abs_path.clone()) {
            Ok(t) => t,
            Err(e) => (), /*return Err(Error::IOError(Box::new(e)))*/
        }

        let mut file = std::fs::File::create(abs_path).to_res()?;

        let j = match serde_json::to_string_pretty(self.settings.res()?) {
            Ok(t) => t,
            Err(e) => return Err(Error::SerdeJsonError(Box::new(e))),
        };

        file.write_all(j.as_bytes());

        Ok(())
    }
}
