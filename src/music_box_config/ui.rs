// ratatui
use ratatui::{
    layout::{self, Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

// Internal
use super::MusicBoxConfig;

pub fn ui(f: &mut Frame, app: &MusicBoxConfig) {
    let settings_max_length = crate::settings::Settings::get_longest_name_len();

    let chunks_main = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(f.size());

    let chunks_sub = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            // length of the largest item plus 2 for the borders
            Constraint::Length(settings_max_length as u16 + 2u16),
            Constraint::Min(1),
        ])
        .split(chunks_main[1]);

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "Music box configurator",
        Style::default().fg(Color::White),
    ))
    .block(title_block)
    .alignment(Alignment::Center);

    let settings_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let settings = Paragraph::new(Text::styled(
        "Here be settings",
        Style::default().fg(Color::White),
    ))
    .block(settings_block)
    .alignment(Alignment::Left);

    f.render_widget(title, chunks_main[0]);
    f.render_widget(settings, chunks_sub[0]);
}

pub enum ValueType {
    Number,
    Colour,
}

pub mod ui_macro {
    // These macros let you configure the ui from the settings.rs file.

    #[macro_export]
    /// This needs to be called with all items you want to add
    /// See settings.rs
    macro_rules! ui_macro_list_items {
        ($enclosing:ty, $($item:ident), +) => {
            pub fn get_items() -> Vec<(String, String, ValueType)> {
                paste::paste! {
                    let items = vec![
                        $(
                            $enclosing::[<get_ $item _name>](),
                        )+
                    ];
                }
                return items;
            }

            pub fn get_longest_name_len() -> usize {
                paste::paste! {
                    let item = vec![
                        $(
                            $enclosing::[<get_ $item _name>](),
                        )+
                    ];

                    let mut len = 0usize;
                    for i in item.iter() {
                        let count = i.1.chars().count();
                        if count > len {
                            len = count;
                        }
                    }
                    len
                }
            }
        };
    }

    #[macro_export]
    /// This needs to be called with all items you want to add to a group
    /// See settings.rs
    macro_rules! ui_macro_add_item {
        ($self:ident, $group:literal, $($item:ident, $value_type_enum:expr, $value_type:ident), +) => {
            $(
                paste::paste! {
                    fn [<get_ $item _name>] () -> (String, String, ValueType) {
                        return ($group.to_string(), stringify!($item).to_string(), $value_type_enum);
                    }

                    fn [<get_ $item _value>] (&$self) -> $value_type {
                        return $self.$item.clone();
                    }

                    fn [<set_ $item _value>] (&mut $self, val: $value_type) {
                        $self.$item = val;
                    }
                }
            )+
        };
    }
}
