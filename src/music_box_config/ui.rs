use std::rc::Rc;

// ratatui
use ratatui::{
    layout::{self, Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, Paragraph, Widget, Wrap},
    Frame,
};

use crate::music_box_config::config_groups::GroupListTrait;

// Internal
use super::MusicBoxConfig;

pub fn ui(f: &mut Frame, app: &mut MusicBoxConfig) {
    // Update liststate
    app.list_state.select(Some(app.settings_index));

    // Set the maximum length which is used for the key input.
    app.settings_arr_length = app.groups.len() - 1;
    app.groups
        .iter()
        .for_each(|x| app.settings_arr_length += x.items.len());

    // Chunks
    let (chunks_main, chunks_sub, chunks_sub_sub) =
        // max_char_length = length of the largest item. This panics if there are no settings. Unwrap is okay.
        chunks(f.size(), app.groups.max_length().unwrap());

    // Block
    let block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default())
        .border_type(BorderType::Rounded);

    // Title
    f.render_widget(
        title().block(block.clone().borders(Borders::BOTTOM)),
        chunks_main[0],
    );

    // Editor (The part where text pops up if you press keys).
    f.render_widget(
        editor(app).block(block.clone().title("Editor")),
        chunks_sub_sub[0],
    );

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

    let list = List::new(list)
        .block(block.clone().title("Settings"))
        //.highlight_style(Style::default().underlined())
        .highlight_symbol(">>");

    f.render_stateful_widget(list, chunks_sub[0], &mut app.list_state);

    // Check for popup
    // Shamelessly stolen from https://github.com/fdehau/tui-rs/blob/master/examples/popup.rs
    if app.parse_error {
        let block = Block::default()
            .title("Error")
            .borders(Borders::ALL)
            .title_alignment(Alignment::Center);
        let area = centered_rect(60, 20, f.size());
        let pop_text = Paragraph::new(vec![
            Line::from("The value you inputted is not a valid float."),
            Line::from("An Example of a valid float would be any integer ('50') or a two integers seperated by a period ('50.1')."),
            Line::from("Enter to continue..."),
        ]).block(block).wrap(Wrap { trim: false });

        let area_sub = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(1), Constraint::Max(1)])
            .split(area);
        f.render_widget(Clear, area);
        f.render_widget(pop_text, area);
    }

    // Save file popup
    if let Some(t) = &app.save_file {
        let block = Block::default()
            .title("Save")
            .borders(Borders::ALL)
            .title_alignment(Alignment::Center);
        let area = centered_rect(60, 20, f.size());
        let pop_text = Paragraph::new(vec![
            Line::from("Saving to:"),
            Line::from("->".to_string() + t.as_str()),
            Line::from("Escape to close dialogue. Enter to save."),
        ])
        .block(block)
        .wrap(Wrap { trim: false });
        let area_sub = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(1), Constraint::Max(1)])
            .split(area);
        f.render_widget(Clear, area);
        f.render_widget(pop_text, area);
    }

    // Open file popup
    if let Some(t) = &app.open_file {
        let block = Block::default()
            .title("Open")
            .borders(Borders::ALL)
            .title_alignment(Alignment::Center);
        let area = centered_rect(60, 20, f.size());
        let pop_text = Paragraph::new(vec![
            Line::from("Opening:"),
            Line::from("->".to_string() + t.as_str()),
            Line::from("Escape to close dialogue. Enter to open."),
        ])
        .block(block)
        .wrap(Wrap { trim: false });
        let area_sub = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(1), Constraint::Max(1)])
            .split(area);
        f.render_widget(Clear, area);
        f.render_widget(pop_text, area);
    }
}

fn chunks(a: Rect, max_char_length: usize) -> (Rc<[Rect]>, Rc<[Rect]>, Rc<[Rect]>) {
    let chunks_main = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Max(2), Constraint::Min(1), Constraint::Max(3)])
        .split(a);

    let chunks_sub = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            // Plus two for the borders.
            Constraint::Max(max_char_length as u16 + 2u16),
            Constraint::Min(1),
        ])
        .split(chunks_main[1]);

    let chunks_sub_sub = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Max(3), Constraint::Min(1)])
        .split(chunks_sub[1]);

    (chunks_main, chunks_sub, chunks_sub_sub)
}

fn title() -> Paragraph<'static> {
    Paragraph::new(Text::styled("Music box configurator", Style::default()))
        .alignment(Alignment::Center)
}

fn editor(app: &MusicBoxConfig) -> Paragraph<'_> {
    Paragraph::new(Text::styled(&app.input_buf, Style::default()))
}

fn navbar() -> Paragraph<'static> {
    Paragraph::new(vec![
        Line::from("^S Save | ^O Open | ^X eXit"),
        Line::from("^L delete Line | "),
    ])
    .alignment(Alignment::Center)
}

fn centered_rect(x: u16, y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - y) / 2),
                Constraint::Percentage(y),
                Constraint::Percentage((100 - y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - x) / 2),
                Constraint::Percentage(x),
                Constraint::Percentage((100 - x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
