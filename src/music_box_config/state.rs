// std
use std::io::{BufReader, Write};

// Internal
use super::{
    functions::MainLoopAction,
    item_list::value::{ValueType, ValueWrapper},
    key_handler::KeyPressEvent,
    ExlusiveBuffers, MusicBoxConfig,
};
use crate::{prelude::*, settings::Settings};

/// The current state of the application
#[derive(Debug, Default, PartialEq, Eq)]
pub enum ApplicationState {
    /// Basic application state. No popups are open
    #[default]
    Default,
    /// An error occured
    GeneralError,
    /// An error occured
    OpenError,
    /// An error occured
    SaveError,
    /// An error occured trying to parse a float
    ParseErrorFloat,
    /// An error occured trying to parse a bool, this should not occur and is a serious error
    ParseErrorBool,
    /// Open file dialogue
    OpenDialogue,
    /// Save file dialogue
    SaveDialogue,
}

/// Something changed. Don't skip to next loop iteration
const NOTHING: MainLoopAction = MainLoopAction::Nothing;
/// Nothing's changed. Skip to the next loop, don't redraw
const CONTINUE: MainLoopAction = MainLoopAction::Continue;
/// Break the loop, exiting the program
const BREAK: MainLoopAction = MainLoopAction::Break;

impl MusicBoxConfig {
    #[inline]
    pub fn change_state(&mut self) -> MainLoopAction {
        let ret = match self.key_press_event {
            KeyPressEvent::None => CONTINUE,

            KeyPressEvent::Exit => self.cs_exit(),
            KeyPressEvent::ClearLine => self.cs_clear(),
            KeyPressEvent::Save => self.cs_save(),
            KeyPressEvent::Open => self.cs_open(),
            KeyPressEvent::Up => self.cs_up(),
            KeyPressEvent::Down => self.cs_down(),

            KeyPressEvent::Char(character) => self.cs_char(character),
            KeyPressEvent::String(ref string) => self.cs_string(string.clone()),
            KeyPressEvent::Backspace => self.cs_backspace(),

            KeyPressEvent::Enter => self.cs_enter(),
            KeyPressEvent::Escape => self.cs_escape(),
        };
        self.key_press_event = KeyPressEvent::None;
        ret
    }

    #[inline]
    fn cs_exit(&mut self) -> MainLoopAction {
        if self.state == ApplicationState::Default {
            BREAK
        } else {
            self.state = ApplicationState::Default;
            NOTHING
        }
    }

    #[inline]
    fn cs_clear(&mut self) -> MainLoopAction {
        if self.state == ApplicationState::Default {
            self.buffers.editor_buffer = String::new();
            return NOTHING;
        }
        CONTINUE
    }

    #[inline]
    fn cs_save(&mut self) -> MainLoopAction {
        if self.state == ApplicationState::Default {
            self.state = ApplicationState::SaveDialogue;
            self.buffers.exlusive_buffer =
                ExlusiveBuffers::SaveFile(self.buffers.path_buffer.clone());
            return NOTHING;
        }
        CONTINUE
    }

    #[inline]
    fn cs_open(&mut self) -> MainLoopAction {
        if self.state == ApplicationState::Default {
            self.state = ApplicationState::OpenDialogue;
            self.buffers.exlusive_buffer =
                ExlusiveBuffers::OpenFile(self.buffers.path_buffer.clone());
            return NOTHING;
        }
        CONTINUE
    }

    #[inline]
    fn cs_up(&mut self) -> MainLoopAction {
        if (self.state != ApplicationState::Default) || (self.index == 0) {
            return CONTINUE;
        }

        let next_wrapper = self.settings.get(self.index - 1);

        if self.cs_write_to_settings() == MainLoopAction::Nothing {
            return NOTHING;
        }

        self.index -= 1;

        if let Some(t) = next_wrapper {
            self.buffers.editor_buffer = t.to_string();
            return CONTINUE;
        }

        self.buffers.editor_buffer = self.lang_map.val_at("capital.groupBuffer.fullStop");
        NOTHING
    }

    #[inline]
    fn cs_down(&mut self) -> MainLoopAction {
        if (self.state != ApplicationState::Default) || (self.index == self.max_index) {
            return CONTINUE;
        }

        let next_wrapper = self.settings.get(self.index + 1);

        if self.cs_write_to_settings() == MainLoopAction::Nothing {
            return NOTHING;
        }

        self.index += 1;

        if let Some(t) = next_wrapper {
            self.buffers.editor_buffer = t.to_string();
            return CONTINUE;
        }

        self.buffers.editor_buffer = self.lang_map.val_at("capital.groupBuffer.fullStop");
        NOTHING
    }

    #[inline]
    fn cs_char(&mut self, character: char) -> MainLoopAction {
        match self.state {
            ApplicationState::Default => {
                self.buffers.editor_buffer.push(character);
                NOTHING
            }
            ApplicationState::OpenDialogue => {
                if let ExlusiveBuffers::OpenFile(mut open_buffer) =
                    self.buffers.exlusive_buffer.clone()
                {
                    open_buffer.push(character);
                } else {
                    self.buffers.exlusive_buffer = ExlusiveBuffers::OpenFile(
                        self.buffers.path_buffer.clone() + &character.to_string(),
                    );
                }
                NOTHING
            }
            ApplicationState::SaveDialogue => {
                if let ExlusiveBuffers::SaveFile(mut save_buffer) =
                    self.buffers.exlusive_buffer.clone()
                {
                    save_buffer.push(character);
                } else {
                    self.buffers.exlusive_buffer = ExlusiveBuffers::SaveFile(
                        self.buffers.path_buffer.clone() + &character.to_string(),
                    );
                }
                NOTHING
            }
            _ => CONTINUE,
        }
    }

    #[inline]
    fn cs_string(&mut self, string: String) -> MainLoopAction {
        match self.state {
            ApplicationState::Default => {
                self.buffers.editor_buffer += &string;
                NOTHING
            }
            ApplicationState::OpenDialogue => {
                if let ExlusiveBuffers::OpenFile(mut open_buffer) =
                    self.buffers.exlusive_buffer.clone()
                {
                    open_buffer += &string
                } else {
                    self.buffers.exlusive_buffer =
                        ExlusiveBuffers::OpenFile(self.buffers.path_buffer.clone() + &string);
                }
                NOTHING
            }
            ApplicationState::SaveDialogue => {
                if let ExlusiveBuffers::SaveFile(mut save_buffer) =
                    self.buffers.exlusive_buffer.clone()
                {
                    save_buffer += &string;
                } else {
                    self.buffers.exlusive_buffer =
                        ExlusiveBuffers::SaveFile(self.buffers.path_buffer.clone() + &string);
                }
                NOTHING
            }
            _ => CONTINUE,
        }
    }

    #[inline]
    fn cs_backspace(&mut self) -> MainLoopAction {
        match self.state {
            ApplicationState::Default => {
                _ = self.buffers.editor_buffer.pop();
                NOTHING
            }
            ApplicationState::OpenDialogue => {
                if let ExlusiveBuffers::OpenFile(ref mut open_buffer) = self.buffers.exlusive_buffer
                {
                    open_buffer.pop();
                } else {
                    let mut new_buffer = self.buffers.path_buffer.clone();
                    new_buffer.pop();
                    self.buffers.exlusive_buffer = ExlusiveBuffers::OpenFile(new_buffer);
                };
                NOTHING
            }
            ApplicationState::SaveDialogue => {
                if let ExlusiveBuffers::SaveFile(ref mut save_buffer) = self.buffers.exlusive_buffer
                {
                    save_buffer.pop();
                } else {
                    let mut new_buffer = self.buffers.path_buffer.clone();
                    new_buffer.pop();
                    self.buffers.exlusive_buffer = ExlusiveBuffers::SaveFile(new_buffer);
                };
                NOTHING
            }
            _ => CONTINUE,
        }
    }

    #[inline]
    fn cs_enter(&mut self) -> MainLoopAction {
        match self.state {
            ApplicationState::Default => (),
            ApplicationState::OpenDialogue => {
                self.cs_open_file();
                self.state = ApplicationState::Default;
            }
            ApplicationState::SaveDialogue => {
                self.cs_save_file();
                self.state = ApplicationState::Default;
            }
            _ => {
                self.state = ApplicationState::Default;
                self.buffers.exlusive_buffer = ExlusiveBuffers::None;
            }
        };
        NOTHING
    }

    #[inline]
    fn cs_escape(&mut self) -> MainLoopAction {
        self.state = ApplicationState::Default;
        CONTINUE
    }

    /// Writes the currently selected option to [MusicBoxConfig::settings]
    #[inline]
    fn cs_write_to_settings(&mut self) -> MainLoopAction {
        let (value_type, index) = (self.settings_item_list[self.index].value_type, self.index);

        let wrapper = match value_type {
            ValueType::None => None,
            ValueType::Colour => Some(ValueWrapper::String(self.buffers.editor_buffer.clone())),
            ValueType::Number => match self.buffers.editor_buffer.parse() {
                Ok(t) => Some(ValueWrapper::F64(t)),
                Err(_) => {
                    self.state = ApplicationState::ParseErrorFloat;
                    return NOTHING;
                }
            },
            ValueType::Boolean => match self.buffers.editor_buffer.parse() {
                Ok(t) => Some(ValueWrapper::Boolean(t)),
                Err(e) => {
                    self.state = ApplicationState::ParseErrorBool;
                    self.buffers.error_buffer = Box::new(Error::Displayable(format!(
                        "{0}{4}{2}{1}{3}",
                        self.lang_map.val_at("capital.invalidBool"),
                        self.buffers.editor_buffer.clone(),
                        self.lang_map.val_at("quoteDelimiterOpen"),
                        self.lang_map.val_at("quoteDelimiterClose"),
                        self.lang_map.val_at("colon.space"),
                    )));
                    return NOTHING;
                }
            },
        };

        if let Some(t) = wrapper {
            self.settings.set(self.index, &t);
        };

        CONTINUE
    }

    /// This should only run if enter has been pressed in the save file dialogue
    #[inline]
    fn cs_save_file(&mut self) -> MainLoopAction {
        if let ExlusiveBuffers::SaveFile(ref save_path) = self.buffers.exlusive_buffer {
            let mut abs_path = match crate::path::absolute_path(save_path.clone()) {
                Ok(t) => t,
                Err(e) => {
                    self.buffers.error_buffer =
                        Box::new(Error::Displayable(e.to_string() + &save_path));
                    return NOTHING;
                }
            };

            let parent = abs_path.parent();

            if let Some(parent_path) = parent {
                std::fs::create_dir_all(parent_path).unwrap_or(());
            };

            let mut file_handle = match std::fs::File::create(abs_path) {
                Ok(t) => t,
                Err(e) => {
                    self.state = ApplicationState::SaveError;
                    self.buffers.error_buffer =
                        Box::new(Error::Displayable(e.to_string() + &save_path));
                    return NOTHING;
                }
            };

            let json = match serde_json::to_string_pretty(&self.settings) {
                Ok(t) => t,
                Err(e) => {
                    self.state = ApplicationState::GeneralError;
                    self.buffers.error_buffer = Box::new(Error::Displayable(e.to_string()));
                    return NOTHING;
                }
            };

            file_handle.write_all(json.as_bytes());
        }

        CONTINUE
    }

    /// This should only run if enter has been pressed in the open file dialogue
    #[inline]
    fn cs_open_file(&mut self) -> MainLoopAction {
        if let ExlusiveBuffers::OpenFile(ref open_path) = self.buffers.exlusive_buffer {
            let mut abs_path = match crate::path::absolute_path(open_path.clone()) {
                Ok(t) => t,
                Err(e) => {
                    self.buffers.error_buffer =
                        Box::new(Error::Displayable(e.to_string() + &open_path));
                    return NOTHING;
                }
            };

            let mut file_handle = match std::fs::File::open(abs_path) {
                Ok(t) => t,
                Err(e) => {
                    self.state = ApplicationState::OpenError;
                    self.buffers.error_buffer =
                        Box::new(Error::Displayable(e.to_string() + &open_path));
                    return NOTHING;
                }
            };

            let deserialized: Settings = match serde_json::from_reader(BufReader::new(file_handle))
            {
                Ok(t) => t,
                Err(e) => {
                    self.state = ApplicationState::GeneralError;
                    self.buffers.error_buffer = Box::new(Error::Displayable(e.to_string()));
                    return NOTHING;
                }
            };

            self.settings = deserialized;
        }

        CONTINUE
    }
}
