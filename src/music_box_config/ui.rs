use std::rc::Rc;

// ratatui
use ratatui::{
    layout::{self, Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Widget},
    Frame,
};

use crate::music_box_config::config_groups::GroupListTrait;

// Internal
use super::MusicBoxConfig;

pub fn ui(f: &mut Frame, app: &mut MusicBoxConfig) {
    // Set the maximum length which is used for the key input
    app.current_setting_max_length = app.groups.len() - 1;
    app.groups
        .iter()
        .for_each(|x| app.current_setting_max_length += x.items.len());

    // Chunks
    let (chunks_main, chunks_sub, chunks_sub_sub) =
        chunks(f.size(), app.groups.max_length().unwrap());

    // Block
    let block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default())
        .border_type(BorderType::Rounded);

    // Title
    f.render_widget(title().block(block.clone()), chunks_main[0]);

    // Navbar
    f.render_widget(
        navbar().block(block.clone().borders(Borders::TOP)),
        chunks_main[2],
    );

    // Settings
    let mut list = Vec::<ListItem>::new();

    for group in app.groups.clone() {
        list.push(ListItem::new(Line::from(
            Span::raw(group.name.clone()).bold(),
        )));
        for item in group.items {
            list.push(ListItem::new(Line::from(
                Span::raw(item.human_readable_name.clone()).fg(Color::White),
            )))
        }
    }

    list[app.current_setting] = list[app.current_setting].clone().underlined();

    let list = List::new(list).block(block.clone());

    f.render_stateful_widget(list, chunks_sub[0], &mut app.list_state);
}

fn chunks(a: Rect, max_char_length: usize) -> (Rc<[Rect]>, Rc<[Rect]>, Rc<[Rect]>) {
    let chunks_main = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Max(3), Constraint::Min(1), Constraint::Max(2)])
        .split(a);

    let chunks_sub = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            // length of the largest item plus 2 for the borders. This panics if there are no settings. Unwrap is okay
            Constraint::Max(max_char_length as u16 + 2u16),
            Constraint::Min(1),
        ])
        .split(chunks_main[1]);

    let chunks_sub_sub = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Max(1), Constraint::Min(1)])
        .split(chunks_sub[1]);

    (chunks_main, chunks_sub, chunks_sub_sub)
}

fn title<'a>() -> Paragraph<'a> {
    Paragraph::new(Text::styled(
        "Music box configurator",
        Style::default().fg(Color::White),
    ))
    .alignment(Alignment::Center)
}

fn navbar<'a>() -> Paragraph<'a> {
    Paragraph::new(Span::styled(
        "^S Save | ^O Open | ^Q Quit",
        Style::default(),
    ))
    .alignment(Alignment::Center)
}
