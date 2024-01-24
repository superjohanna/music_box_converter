// std
use std::{
    fs::File,
    io::{stdout, BufReader, Stdout},
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
use serde::Serialize;

// Internal
use super::{ui::ui, MusicBoxConfig};
use crate::{prelude::*, settings::Settings};

impl MusicBoxConfig {
    pub fn run(&mut self) -> Result<()> {
        stdout().execute(EnterAlternateScreen).to_res()?;
        enable_raw_mode().to_res()?;
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout())).to_res()?;
        terminal.clear().to_res()?;

        self.load_settings();

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

            #[allow(clippy::collapsible_if)]
            if event::poll(std::time::Duration::from_millis(100)).to_res()? {
                if let Event::Key(key) = event::read().to_res()? {
                    if !(key.kind == KeyEventKind::Press) {
                        continue;
                    }
                    if key.modifiers == KeyModifiers::CONTROL {
                        match key.code {
                            KeyCode::Char('x') => break,
                            KeyCode::Char('l') => self.value_input = String::new(),
                            _ => (),
                        }
                    } else if key.modifiers == KeyModifiers::SHIFT {
                        match key.code {
                            KeyCode::Char(c) => {
                                self.value_input += &c.to_uppercase().collect::<String>()
                            }
                            KeyCode::Backspace => {
                                self.value_input.pop().res()?;
                            }
                            _ => continue,
                        }
                    } else if key.modifiers == KeyModifiers::NONE {
                        match key.code {
                            KeyCode::Char(c) => self.value_input.push(c),
                            KeyCode::Backspace => {
                                self.value_input.pop().res()?;
                            }
                            KeyCode::Up => {
                                if self.settings_index != 0 {
                                    self.settings_index -= 1;
                                    self.update_settings_index();
                                }
                            }
                            KeyCode::Down => {
                                if self.settings_index != self.settings_arr_length {
                                    self.settings_index += 1;
                                    self.update_settings_index();
                                }
                            }
                            _ => continue,
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn update_settings_index(&mut self) -> Result<()> {
        // Return if we are at a group. After all we can't write to a group
        if self.settings_index_bool[self.settings_index] {
            return Ok(());
        }
        // Sadly we need to do this. It's not optimal but I don't want to delete everything.
        // This kinda implies that my code is usually optimal which it certainly is not. haha  🫠
        let mut index = 0usize;
        for group in &self.groups {
            index += 1;
            if self.settings_index == index {
                //panic!("This code should be unreachable. Something went terribly wrong. update_settings_index()");
            }
            for item in group.items.iter() {
                index += 1;
                if self.settings_index == index {
                    // AHHH This doens't work ;(
                }
            }
        }
        Ok(())
    }

    /// Deserializes ./svg_settings.json and assigns the deserialized SvgSettings to self.svg_settings.
    fn load_settings(&mut self) -> Result<()> {
        let file = match File::open(self.args.get_one::<String>("io_settings").unwrap()) {
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
            Err(e) => return Err(Error::IOError(Box::new(e))),
        }

        let file = std::fs::File::create(abs_path).to_res()?;

        let serializer = serde_json::Serializer::new(file);

        Ok(())
    }
}
