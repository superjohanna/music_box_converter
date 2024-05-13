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
use super::{area::Areas, key_handler::KeyPressEvent};
use super::{state::ApplicationState, ExlusiveBuffers, MusicBoxConfig};
use crate::{prelude::*, settings::Settings};

#[derive(Debug, Default, PartialEq, Eq)]
pub enum MainLoopAction {
    /// Something changed. Don't skip to next loop iteration
    #[default]
    Nothing,
    /// Nothing's changed. Skip to the next loop, don't redraw
    Continue,
    /// Break the loop, exiting the program
    Break,
}

impl MusicBoxConfig {
    pub fn run(&mut self) -> Result<()> {
        stdout().execute(EnterAlternateScreen).to_res()?;
        enable_raw_mode().to_res()?;
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout())).to_res()?;
        terminal.clear().to_res()?;

        // Act as if the user is trying to open a file
        self.buffers.exlusive_buffer = ExlusiveBuffers::OpenFile(self.buffers.path_buffer.clone());
        self.state = ApplicationState::OpenDialogue;
        self.key_press_event = KeyPressEvent::Enter;
        self.change_state();
        let result = self.main_loop(&mut terminal);

        stdout().execute(LeaveAlternateScreen).to_res()?;
        disable_raw_mode().to_res()?;
        result?;
        Ok(())
    }

    fn main_loop(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
        loop {
            // Not using terminal.draw instead doing everything it does manually
            // Also copying comments

            // -- Read key and parse it to a better format --
            if event::poll(std::time::Duration::from_millis(100)).to_res()? {
                if let Event::Key(key) = event::read().to_res()? {
                    self.handle_key(&key)
                }
            };

            // -- Calculate the sizes and check if we need to redraw --

            // Autoresize - otherwise we get glitches if shrinking or potential desync between widgets
            // and the terminal (if growing), which may OOB.
            terminal.autoresize().to_res()?;

            let mut frame = terminal.get_frame();

            let size = frame.size();
            let area_old = self.area.clone();
            self.area = Some(
                match Areas::split_area(
                    size,
                    self.settings_item_list.longest_human_readable_name_length,
                ) {
                    Ok(t) => t,
                    Err(Error::TerminalTooSmall) => {
                        return Err(Error::Generic(
                            self.lang_map.val_at("terminalTooSmall").to_owned(),
                        ))
                    }
                    _ => panic!("Unhandled Error"),
                },
            );

            // Old area and new area are equal which means the terminal didn't change size
            let no_change = self.area == area_old;

            // -- Act on the key --
            let action = self.change_state();

            // Check if there was a state change if there was no size change
            if no_change {
                match action {
                    MainLoopAction::Nothing => (),
                    MainLoopAction::Continue => continue,
                    MainLoopAction::Break => break,
                }
            }

            // Draw UI
            self.ui(&mut frame);
            terminal.flush();

            terminal.hide_cursor();

            terminal.swap_buffers();
            terminal.backend_mut().flush();
        }
        Ok(())
    }
}
