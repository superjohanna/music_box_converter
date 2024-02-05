use std::rc::Rc;

// ratatui
use ratatui::{
    layout::{self, Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, Paragraph, Widget, Wrap},
    Frame,
};

use self::value::ValueType;

// Internal
use super::MusicBoxConfig;
use super::item_list::*;

pub fn ui(f: &mut Frame, app: &mut MusicBoxConfig) {
    // Update liststate
    app.list_state.select(Some(app.index));

    // Set the maximum length which is used for the key input.
    app.max_index = app.settings_item_list.len() - 1;

    // Chunks
    let (terminal_chunks, mid_section_chunks, mid_right_chunks) =
        get_chunks(f.size(), app.settings_item_list.longest_human_readable_name_length);

    // Block
    let block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default())
        .border_type(BorderType::Rounded);

    // Title
    f.render_widget(
        get_title().block(block.clone().borders(Borders::BOTTOM)),
        terminal_chunks[0],
    );

    // Editor (The part where text pops up if you press keys).
    f.render_widget(
        get_editor(app).block(block.clone().title("Editor")),
        mid_right_chunks[0],
    );

    // Tip
    let tip = get_tip(app);

    let help = if !app.settings_item_list[app.index].help.is_empty() {
        Line::from(vec![
            Span::from("Help: ").bold(),
            Span::from(app.settings_item_list[app.index].help.clone()),
        ])
    } else {
        Line::from("")
    };

    let tip_and_help = Paragraph::new(vec![tip, Line::from(""), help]);

    f.render_widget(tip_and_help.wrap(Wrap { trim: false }), mid_right_chunks[1]);

    // Navbar
    f.render_widget(
        get_navbar().block(block.clone().borders(Borders::TOP)),
        terminal_chunks[2],
    );

    // Settings
    let mut list = Vec::<ListItem>::new();

    for item in app.settings_item_list.iter() {
        if let ValueType::None = item.value_type {
            list.push(ListItem::new(item.human_name.clone()).bold())
        } else {
            list.push(ListItem::new(item.human_name.clone()));
        }
    }

    let list = List::new(list)
        .block(block.clone().title("Settings"))
        .highlight_symbol(">>");

    f.render_stateful_widget(list, mid_section_chunks[0], &mut app.list_state);

    // Check for popup
    // Shamelessly stolen from https://github.com/fdehau/tui-rs/blob/master/examples/popup.rs
    if app.parse_error {
        let block = Block::default()
            .title("Error")
            .borders(Borders::ALL)
            .title_alignment(Alignment::Center);
        let area = centered_rect_helper(60, 20, f.size());
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

    // Save error popup
    if app.save_error.is_some() {
        let error =  app.save_error.as_ref().unwrap().to_string();
        let block = Block::default()
            .title("Error")
            .borders(Borders::ALL)
            .title_alignment(Alignment::Center);
        let area = centered_rect_helper(60, 20, f.size());
        let pop_text = Paragraph::new(vec![
            Line::from(format!("Couldn't save file to '{}'", app.path_buf)),
            Line::from(error),
        ]).block(block).wrap(Wrap { trim: false });

        let area_sub = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(1), Constraint::Max(1)])
            .split(area);
        f.render_widget(Clear, area);
        f.render_widget(pop_text, area);
    }

    // Open error popup
    if app.open_error.is_some() {
        let error =  app.open_error.as_ref().unwrap().to_string();
        let block = Block::default()
            .title("Error")
            .borders(Borders::ALL)
            .title_alignment(Alignment::Center);
        let area = centered_rect_helper(60, 20, f.size());
        let pop_text = Paragraph::new(vec![
            Line::from(format!("Couldn't open file '{}'", app.path_buf)),
            Line::from(error),
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
        let area = centered_rect_helper(60, 20, f.size());
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
        let area = centered_rect_helper(60, 20, f.size());
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

fn get_tip(app: &MusicBoxConfig) -> Line<'_> {
    match app.settings_item_list[app.index].value_type {
        ValueType::None => {
            Line::from(vec![
                Span::from("Tip: ").bold(),
                Span::from("This is a group. Editing it does nothing. It's only here for organization."),
                ])
        },
        ValueType::Number => {
        
            Line::from(vec![
                Span::from("Tip: ").bold(),
                Span::from("This is a floating point number. It can be an integer or two integers seperated by a period (50 and 50.0 are the same)."),
                ])
        },
        ValueType::Colour => {
            Line::from(vec![
                Span::from("Tip: ").bold(),
                Span::from("This is a colour. You can use hex notation (#ffffff for white) or rgb notation (rgb(255, 255, 255) for white) or any other svg supported format."),
            ])
        },
        ValueType::Boolean => {
            Line::from(vec![
                Span::from("Tip: ").bold(),
                Span::from("This is a checkbox. You can toggle it on or off with the spacebar."),
            ])
        },
    }
}

#[allow(clippy::type_complexity)]
fn get_chunks(a: Rect, max_char_length: usize) -> (Rc<[Rect]>, Rc<[Rect]>, Rc<[Rect]>) {
    let chunks_main = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Max(2), Constraint::Min(1), Constraint::Max(3)])
        .split(a);

    let chunks_sub = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            // Plus two for the borders and two extra for the list selection symbol '>>'
            Constraint::Max(max_char_length as u16 + 4u16),
            Constraint::Min(1),
        ])
        .split(chunks_main[1]);

    let mut chunks_sub_sub = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Max(3), Constraint::Min(1)])
        .split(chunks_sub[1]);

    // You may be asking what this is. Good question
    // Basically we want to make a little space to the left of chunks_sub_sub[1] but Rc<[Rect]> doesn't implement DerefMut so we create a new one
    let chunks_sub_sub = Rc::new([
        chunks_sub_sub[0],
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Max(1), Constraint::Min(1)])
            .split(chunks_sub_sub[1])[1],
    ]);

    (chunks_main, chunks_sub, chunks_sub_sub)
}

fn get_title() -> Paragraph<'static> {
    Paragraph::new(Text::styled("Music box configurator", Style::default()))
        .alignment(Alignment::Center)
}

fn get_editor(app: &MusicBoxConfig) -> Paragraph<'_> {
    match app.settings_item_list[app.index].value_type {
        ValueType::Boolean => {
            // The windows console can't render the unicode characters I wanted to use ;(
            if &app.input_buf == "false" {
                Paragraph::new(Text::styled("Off", Style::default()))
            } else {
                Paragraph::new(Text::styled("On", Style::default()))
            }
        },
        _ => Paragraph::new(Text::styled(&app.input_buf, Style::default())),
    }
}

fn get_navbar() -> Paragraph<'static> {
    Paragraph::new(vec![
        Line::from("^S Save | ^O Open | ^X eXit"),
        Line::from("^L delete Line | ^E move up | ^D move down"),
    ])
    .alignment(Alignment::Center)
}

fn centered_rect_helper(x: u16, y: u16, r: Rect) -> Rect {
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
