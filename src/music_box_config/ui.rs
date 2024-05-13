// ratatui
use ratatui::{
    layout::Alignment,
    style::{Style, Stylize},
    symbols::line::DOUBLE_VERTICAL_LEFT,
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, Paragraph, Widget, Wrap},
    Frame,
};

// Internal
use super::{item_list::value::ValueType, state::ApplicationState, MusicBoxConfig};
use crate::prelude::*;

const DEFAULT_BLOCK: Block = Block::new()
    .borders(Borders::ALL)
    .style(Style::new())
    .border_type(BorderType::Rounded);

const NO_TRIM: Wrap = Wrap { trim: false };

impl MusicBoxConfig {
    pub fn ui(&mut self, frame: &mut Frame) -> Result<()> {
        self.list_state.select(Some(self.index));

        // Render Title
        frame.render_widget(self.title(), self.area.res()?.terminal_chunks[0]);

        // Render Editor (The part where text pops up if you press a key)
        frame.render_widget(self.editor(), self.area.res()?.editor_chunks[0]);

        // Render Tip & Help
        frame.render_widget(self.tip_and_help(), self.area.res()?.editor_chunks[1]);

        // Render Navbar
        frame.render_widget(self.navbar(), self.area.res()?.terminal_chunks[2]);

        // Render Settings
        let list: Vec<ListItem> = self
            .settings_item_list
            .iter()
            .map(|item| {
                if matches!(item.value_type, ValueType::None) {
                    ListItem::new(item.human_name.to_owned()).bold()
                } else {
                    ListItem::new(item.human_name.to_owned())
                }
            })
            .collect();

        let list = List::new(list).block(DEFAULT_BLOCK).highlight_symbol(">>");

        frame.render_stateful_widget(list, self.area.res()?.main_chunks[0], &mut self.list_state);

        // Popup
        self.popup(frame);

        Ok(())
    }

    fn title(&self) -> Paragraph {
        Paragraph::new(Text::styled(
            self.lang_map.val_at("capital.title"),
            Style::default(),
        ))
        .alignment(Alignment::Center)
        .block(DEFAULT_BLOCK.borders(Borders::BOTTOM))
    }

    fn editor(&self) -> Paragraph {
        match self.settings_item_list[self.index].value_type {
            ValueType::Boolean => {
                // The windows console can't render the unicode characters I wanted to use ;(
                if &self.buffers.editor_buffer == "true" {
                    Paragraph::new(Text::from(self.lang_map.val_at("capital.boolTrue")))
                } else {
                    Paragraph::new(Text::from(self.lang_map.val_at("capital.boolFalse")))
                }
            }
            _ => Paragraph::new(Text::from(self.buffers.editor_buffer.clone())),
        }
        .block(DEFAULT_BLOCK.title(self.lang_map.val_at("capital.editor")))
    }

    fn tip_and_help(&self) -> Paragraph {
        let tip = Line::from(vec![
            Span::from(
                self.lang_map.val_at("capital.tip").to_owned()
                    + self.lang_map.val_at("colon.space"),
            ),
            Span::from(match self.settings_item_list[self.index].value_type {
                ValueType::None => self.lang_map.val_at("capital.groupHint.fullStop"),
                ValueType::Number => self.lang_map.val_at("capital.floatHint.fullStop"),
                ValueType::Colour => self.lang_map.val_at("capital.colourHint.fullStop"),
                ValueType::Boolean => self.lang_map.val_at("capital.checkboxHint.fullStop"),
            }),
        ]);

        let help = if !self.settings_item_list[self.index].help.is_empty() {
            Line::from(vec![
                Span::from(
                    self.lang_map.val_at("capital.help").to_owned()
                        + self.lang_map.val_at("colon.space"),
                )
                .bold(),
                Span::from(self.settings_item_list[self.index].help.clone()),
            ])
        } else {
            Line::from("")
        };

        Paragraph::new(vec![tip, Line::from(""), help])
            .wrap(NO_TRIM)
            .block(DEFAULT_BLOCK.borders(Borders::NONE))
    }

    fn navbar(&self) -> Paragraph {
        Paragraph::new(vec![
            Line::from(format!(
                "{1}{2}{0}{3}{4}{0}{5}{6}",
                self.lang_map.val_at("space.seperator.space"),
                self.lang_map.val_at("caret.save.space"),
                self.lang_map.val_at("capital.save"),
                self.lang_map.val_at("caret.open.space"),
                self.lang_map.val_at("capital.open"),
                self.lang_map.val_at("caret.exit.space"),
                self.lang_map.val_at("capital.exit.space"),
            )),
            Line::from(format!(
                "{1}{2}{0}{3}{4}{0}{5}{6}",
                self.lang_map.val_at("space.seperator.space"),
                self.lang_map.val_at("caret.deleteLine.space"),
                self.lang_map.val_at("capital.deleteLine.space"),
                self.lang_map.val_at("caret.moveUp.space"),
                self.lang_map.val_at("capital.moveUp"),
                self.lang_map.val_at("caret.moveDown.space"),
                self.lang_map.val_at("capital.moveDown"),
            )),
        ])
        .alignment(Alignment::Center)
        .block(DEFAULT_BLOCK.borders(Borders::TOP))
    }

    fn popup(&self, frame: &mut Frame) -> Result<()> {
        let pop_text = Paragraph::new(match self.state {
            ApplicationState::GeneralError => vec![
                Line::from(
                    self.lang_map.val_at("capital.error").to_owned()
                        + self.lang_map.val_at("colon.space"),
                ),
                Line::from(self.buffers.error_buffer.to_string()),
            ],
            ApplicationState::OpenError => vec![
                Line::from(format!(
                    "{0}{2}{1}{3}",
                    self.lang_map.val_at("capital.openFailed.space"),
                    match &self.buffers.exlusive_buffer {
                        super::ExlusiveBuffers::OpenFile(string) => string.clone(),
                        _ => "".to_string(),
                    },
                    self.lang_map.val_at("quoteDelimiterOpen"),
                    self.lang_map.val_at("quoteDelimiterClose")
                )),
                Line::from(self.buffers.error_buffer.to_string()),
            ],
            ApplicationState::SaveError => vec![
                Line::from(format!(
                    "{0}{2}{1}{3}",
                    self.lang_map.val_at("capital.saveFailed.space"),
                    match &self.buffers.exlusive_buffer {
                        super::ExlusiveBuffers::SaveFile(string) => string.clone(),
                        _ => "".to_string(),
                    },
                    self.lang_map.val_at("quoteDelimiterOpen"),
                    self.lang_map.val_at("quoteDelimiterClose")
                )),
                Line::from(self.buffers.error_buffer.to_string()),
            ],
            ApplicationState::ParseErrorFloat => vec![
                Line::from(self.lang_map.val_at("capital.invalidFloat.fullStop")),
                Line::from(self.lang_map.val_at("capital.exampleFloat.fullStop")),
                Line::from(self.lang_map.val_at("capital.continue.ellipsis")),
            ],
            ApplicationState::ParseErrorBool => vec![
                Line::from(self.lang_map.val_at("capital.invalidBool.fullStop")),
                Line::from(self.lang_map.val_at("capital.continue.ellipsis")),
            ],
            ApplicationState::OpenDialogue => vec![
                Line::from(
                    self.lang_map.val_at("capital.openFrom").to_owned()
                        + self.lang_map.val_at("colon.space"),
                ),
                Line::from(
                    self.lang_map.val_at("arrow.space").to_owned()
                        + self.buffers.exlusive_buffer.as_ref().unwrap(),
                ),
                Line::from(self.lang_map.val_at("capital.openHint.fullStop")),
            ],
            ApplicationState::SaveDialogue => vec![
                Line::from(
                    self.lang_map.val_at("capital.saveTo").to_owned()
                        + self.lang_map.val_at("colon.space"),
                ),
                Line::from(
                    self.lang_map.val_at("arrow.space").to_owned()
                        + self.buffers.exlusive_buffer.as_ref().unwrap(),
                ),
                Line::from(self.lang_map.val_at("capital.saveHint")),
            ],
            _ => return Ok(()),
        })
        .block(DEFAULT_BLOCK)
        .wrap(NO_TRIM);

        frame.render_widget(Clear, *self.area.res()?.popup_chunk);
        frame.render_widget(pop_text, *self.area.res()?.popup_chunk);

        Ok(())
    }
}
