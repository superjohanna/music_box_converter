use std::rc::Rc;

// ratatui
use ratatui::{
    layout::{self, Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, Paragraph, Widget, Wrap},
    Frame,
};

use crate::music::music_box::MusicBox;

use self::value::ValueType;

// Internal
use super::item_list::*;
use super::MusicBoxConfig;

pub fn ui(f: &mut Frame, app: &mut MusicBoxConfig) {
    // Update liststate
    app.list_state.select(Some(app.index));

    // Set the maximum length which is used for the key input.
    app.max_index = app.settings_item_list.len() - 1;

    // Chunks
    let (terminal_chunks, mid_section_chunks, mid_right_chunks) = get_chunks(
        f.size(),
        app.settings_item_list.longest_human_readable_name_length,
    );

    // Block
    let block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default())
        .border_type(BorderType::Rounded);

    // Title
    f.render_widget(
        get_title(app).block(block.clone().borders(Borders::BOTTOM)),
        terminal_chunks[0],
    );

    // Editor (The part where text pops up if you press keys).
    f.render_widget(
        get_editor(app).block(block.clone().title(app.lang_map.val_at("capital.editor"))),
        mid_right_chunks[0],
    );

    // Tip
    let tip = get_tip(app);

    let help = if !app.settings_item_list[app.index].help.is_empty() {
        Line::from(vec![
            Span::from(app.lang_map.val_at("capital.help") + &app.lang_map.val_at("colon.space"))
                .bold(),
            Span::from(app.settings_item_list[app.index].help.clone()),
        ])
    } else {
        Line::from("")
    };

    let tip_and_help = Paragraph::new(vec![tip, Line::from(""), help]);

    f.render_widget(tip_and_help.wrap(Wrap { trim: false }), mid_right_chunks[1]);

    // Navbar
    f.render_widget(
        get_navbar(app).block(block.clone().borders(Borders::TOP)),
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
        .block(block.clone().title(app.lang_map.val_at("capital.settings")))
        .highlight_symbol(">>");

    f.render_stateful_widget(list, mid_section_chunks[0], &mut app.list_state);

    // Check for popup
    // Shamelessly stolen from https://github.com/fdehau/tui-rs/blob/master/examples/popup.rs
    if app.parse_error {
        let block = Block::default()
            .title(app.lang_map.val_at("capital.error"))
            .borders(Borders::ALL)
            .title_alignment(Alignment::Center);
        let area = centered_rect_helper(60, 20, f.size());
        let pop_text = Paragraph::new(vec![
            Line::from(app.lang_map.val_at("capital.invalidFloat.fullStop")),
            Line::from(app.lang_map.val_at("capital.exampleFloat.fullStop")),
            Line::from(app.lang_map.val_at("capital.continue.ellipsis")),
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

    // Save error popup
    if app.save_error.is_some() {
        let error = app.save_error.as_ref().unwrap().to_string();
        let block = Block::default()
            .title("Error")
            .borders(Borders::ALL)
            .title_alignment(Alignment::Center);
        let area = centered_rect_helper(60, 20, f.size());
        let pop_text = Paragraph::new(vec![
            Line::from(format!(
                "{0}{2}{1}{3}'",
                app.lang_map.val_at("capital.saveFailed"),
                app.path_buf,
                app.lang_map.val_at("quoteDelimiterOpen"),
                app.lang_map.val_at("quoteDelimiterClose")
            )),
            Line::from(error),
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

    // Open error popup
    if app.open_error.is_some() {
        let error = app.open_error.as_ref().unwrap().to_string();
        let block = Block::default()
            .title(app.lang_map.val_at("capital.error"))
            .borders(Borders::ALL)
            .title_alignment(Alignment::Center);
        let area = centered_rect_helper(60, 20, f.size());
        let pop_text = Paragraph::new(vec![
            Line::from(format!(
                "{0}{2}{1}{3}'",
                app.lang_map.val_at("capital.openFailed"),
                app.path_buf,
                app.lang_map.val_at("quoteDelimiterOpen"),
                app.lang_map.val_at("quoteDelimiterClose")
            )),
            Line::from(error),
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

    // Save file popup
    if let Some(t) = &app.save_file {
        let block = Block::default()
            .title(app.lang_map.val_at("capital.save"))
            .borders(Borders::ALL)
            .title_alignment(Alignment::Center);
        let area = centered_rect_helper(60, 20, f.size());
        let pop_text = Paragraph::new(vec![
            Line::from(app.lang_map.val_at("capital.saveTo") + &app.lang_map.val_at("colon.space")),
            Line::from(app.lang_map.val_at("arrow.space") + t.as_str()),
            Line::from(app.lang_map.val_at("capital.saveHint")),
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
            .title(app.lang_map.val_at("capital.open"))
            .borders(Borders::ALL)
            .title_alignment(Alignment::Center);
        let area = centered_rect_helper(60, 20, f.size());
        let pop_text = Paragraph::new(vec![
            Line::from(
                app.lang_map.val_at("capital.openFrom") + &app.lang_map.val_at("colon.space"),
            ),
            Line::from(app.lang_map.val_at("arrow.space") + t.as_str()),
            Line::from(app.lang_map.val_at("capital.openHint.fullStop")),
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
    Line::from(vec![
        Span::from(app.lang_map.val_at("capital.tip") + &app.lang_map.val_at("colon.space")),
        Span::from(match app.settings_item_list[app.index].value_type {
            ValueType::None => app.lang_map.val_at("capital.groupHint.fullStop"),
            ValueType::Number => app.lang_map.val_at("capital.floatHint.fullStop"),
            ValueType::Colour => app.lang_map.val_at("capital.colourHint.fullStop"),
            ValueType::Boolean => app.lang_map.val_at("capital.checkboxHint.fullStop"),
        }),
    ])
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

fn get_title(app: &MusicBoxConfig) -> Paragraph<'static> {
    Paragraph::new(Text::styled(
        app.lang_map.val_at("capital.title"),
        Style::default(),
    ))
    .alignment(Alignment::Center)
}

fn get_editor(app: &MusicBoxConfig) -> Paragraph<'_> {
    match app.settings_item_list[app.index].value_type {
        ValueType::Boolean => {
            // The windows console can't render the unicode characters I wanted to use ;(
            if &app.input_buf == "false" {
                Paragraph::new(Text::from(app.lang_map.val_at("capital.boolFalse")))
            } else {
                Paragraph::new(Text::from(app.lang_map.val_at("capital.boolTrue")))
            }
        }
        _ => Paragraph::new(Text::from(app.input_buf.clone())),
    }
}

fn get_navbar(app: &MusicBoxConfig) -> Paragraph<'static> {
    Paragraph::new(vec![
        Line::from(format!(
            "{1}{2}{0}{3}{4}{0}{5}{6}",
            app.lang_map.val_at("space.seperator.space"),
            app.lang_map.val_at("caret.save"),
            app.lang_map.val_at("capital.save"),
            app.lang_map.val_at("caret.open"),
            app.lang_map.val_at("capital.open"),
            app.lang_map.val_at("caret.exit"),
            app.lang_map.val_at("capital.exit"),
        )),
        Line::from(format!(
            "{1}{2}{0}{3}{4}{0}{5}{6}",
            app.lang_map.val_at("space.seperator.space"),
            app.lang_map.val_at("caret.deleteLine"),
            app.lang_map.val_at("capital.deleteLine"),
            app.lang_map.val_at("caret.moveUp"),
            app.lang_map.val_at("capital.moveUp"),
            app.lang_map.val_at("caret.moveDown"),
            app.lang_map.val_at("capital.moveDown"),
        )),
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
