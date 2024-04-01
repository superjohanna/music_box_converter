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
use super::key_handler::KeyPressEvent;
use super::{state::ApplicationState, ui::ui, ExlusiveBuffers, MusicBoxConfig};
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
        self.main_loop(&mut terminal);

        stdout().execute(LeaveAlternateScreen).to_res()?;
        disable_raw_mode().to_res()?;
        Ok(())
    }

    fn main_loop(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
        loop {
            // Calculate the sizes and check if we need to draw
            let size_changed = false;
            terminal
                .draw(|frame| {
                    ui(frame, self);
                })
                .to_res()?;

            // Read key and parse it to a better format
            if event::poll(std::time::Duration::from_millis(100)).to_res()? {
                if let Event::Key(key) = event::read().to_res()? {
                    self.handle_key(&key)
                }
            };

            // Act on the key
            if !size_changed {
                match self.change_state() {
                    MainLoopAction::Nothing => (),
                    MainLoopAction::Continue => continue,
                    MainLoopAction::Break => break,
                }
            } else {
                self.change_state();
            }

            // Draw UI
        }
        Ok(())
    }
}
