// crossterm
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

// Internal
use super::{functions::MainLoopAction, MusicBoxConfig};
use crate::prelude::*;

/// An abstraction of the key presses. Get assigned by the key handler
#[derive(Debug, Default, PartialEq, Eq)]
pub enum KeyPressEvent {
    /// Don't change state
    #[default]
    None,

    // Control changes (when the user presses control)
    /// Exit (Ctrl + X)
    Exit,
    /// Clear line (Ctrl + L)
    ClearLine,
    /// Open save dialogue (Ctrl + S)
    Save,
    /// Open open dialogue (Ctrl + O)
    Open,
    /// Go up (Ctrl + E | ArrowUp)
    Up,
    /// Go down (Ctrl + D | ArrowDown)
    Down,

    // Text changes
    /// Character
    Char(char),
    /// Sometimes uppercase characters are longer than one character. for example the german 'ß' (eszett or sharp s) maps to uppercase 'SS' because there is no capital sharp s. Except that there is 'ßẞ' but I guess that doesn't matter. This was introduced in unicode in 2008 but it isn't on any keyboard mapping as far as I know. It certainly isn't on my keyboard.
    String(String),
    /// Backspace
    Backspace,

    /// Enter
    Enter,
    /// Escape
    Escape,
}

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
