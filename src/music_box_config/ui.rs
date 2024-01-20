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
        .constraints([Constraint::Length(10), Constraint::Min(1)])
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

    f.render_widget(title, chunks_main[0])
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
            fn get_items() -> Vec<(String, String, ValueType)> {
                paste::paste! {
                    let items = vec![
                        $(
                            $enclosing::[<get_ $item _name>](),
                        )+
                    ];
                }
                return items;
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
