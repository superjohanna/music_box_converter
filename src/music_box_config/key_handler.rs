// crossterm
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

// Internal
use super::{functions::MainLoopAction, state::KeyPressEvent, MusicBoxConfig};
use crate::prelude::*;

impl MusicBoxConfig {
    #[inline]
    pub fn handle_key(&mut self, key: &KeyEvent) {
        if !(key.kind == KeyEventKind::Press) {
            return;
        }
        match key.modifiers {
            KeyModifiers::CONTROL => self.handle_control(key),
            KeyModifiers::SHIFT => self.handle_shift(key),
            KeyModifiers::NONE => self.handle_none(key),
            _ => (),
        }
    }

    #[inline]
    fn handle_control(&mut self, key: &KeyEvent) {
        self.key_press_event = match key.code {
            KeyCode::Char('x') => KeyPressEvent::Exit,
            KeyCode::Char('l') => KeyPressEvent::ClearLine,
            KeyCode::Char('s') => KeyPressEvent::Save,
            KeyCode::Char('o') => KeyPressEvent::Open,
            KeyCode::Char('e') | KeyCode::Up => KeyPressEvent::Up,
            KeyCode::Char('d') | KeyCode::Down => KeyPressEvent::Down,
            _ => return,
        };
    }

    #[inline]
    fn handle_shift(&mut self, key: &KeyEvent) {
        self.key_press_event = match key.code {
            KeyCode::Char(character) if !character.is_control() => {
                KeyPressEvent::String(character.to_uppercase().collect())
            }
            _ => return,
        };
    }

    #[inline]
    fn handle_none(&mut self, key: &KeyEvent) {
        self.key_press_event = match key.code {
            KeyCode::Char(character) if !character.is_control() => KeyPressEvent::Char(character),
            KeyCode::Up => KeyPressEvent::Up,
            KeyCode::Down => KeyPressEvent::Down,
            KeyCode::Enter => KeyPressEvent::Enter,
            KeyCode::Esc => KeyPressEvent::Escape,
            KeyCode::Backspace => KeyPressEvent::Backspace,
            _ => return,
        };
    }
}
