// std
use std::rc::Rc;

// ratatui
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

// Internal
use crate::prelude::*;

const POPUP_HEIGHT: u16 = 6;
const POPUP_WIDTH: u16 = 128;

/// The different Rects that split the terminal into different areas
/// ```plain
/// |-----------------------------------|
/// |         terminal_chunks[0]        |
/// |-----------------------------------|
/// |         terminal_chunks[1]        |
/// | main_chunks[0] | main_chunks[0]   |
/// |                | editor_chunks[0] |
/// |                |------------------|
/// |                | editor_chunks[1] |
/// |-----------------------------------|
/// |         terminal_chunks[2]        |
/// |-----------------------------------|
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Areas {
    /// Indecies
    /// 0: --Title--
    /// 1: Settings | Editor
    /// 2: --Navbar--
    pub terminal_chunks: Rc<[Rect]>,

    /// This is the area of index 1 of [Blocks::terminal_chunks].
    /// It's basically the main area of the program excluding the title and navbar.
    /// ```plain
    ///    0         1
    ///    |         |
    ///   \|/       \|/
    /// Settings | Editor area
    /// ```
    pub main_chunks: Rc<[Rect]>,

    /// This is the editor area including the text field, the hint and the tip.
    /// It's the area of index 1 of [Blocks::main_chunks].
    /// 0: Editor field
    /// 1: Tip & Help
    pub editor_chunks: Rc<[Rect]>,

    pub popup_chunk: Rc<Rect>,
}

impl Areas {
    pub fn split_area(area: Rect, longest_settings_name_length: usize) -> Result<Areas> {
        let terminal_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                // Title
                Constraint::Max(2),
                // Main area
                Constraint::Min(1),
                // Navbar
                Constraint::Max(3),
            ])
            .split(area);

        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                // Settings list
                // Plus two for the borders and two extra for the list selection characters '>>'
                Constraint::Max(longest_settings_name_length as u16 + 4u16),
                // Editor area
                Constraint::Min(1),
            ])
            // Main area
            .split(terminal_chunks[1]);

        let editor_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                // Editor field
                Constraint::Max(3),
                // Tip & Help
                Constraint::Min(1),
            ])
            .split(main_chunks[1]);

        // Create new rc and shadow original because we need to move the help & tip section one to the right and make it's width one smaller
        let editor_chunks = Rc::new([
            // leave unchanged
            editor_chunks[0],
            // make smaller
            Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Max(1), Constraint::Min(1)])
                .split(editor_chunks[1])[1],
        ]);

        // Create the popup space
        if area.height < POPUP_HEIGHT || area.width < POPUP_WIDTH {
            return Err(Error::TerminalTooSmall);
        }

        let top_bottom_distance = (area.height - POPUP_HEIGHT) / 2;
        let left_right_distance = (area.width - POPUP_WIDTH) / 2;

        let popup_chunks_vertical = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Max(top_bottom_distance),
                Constraint::Max(POPUP_HEIGHT),
                Constraint::Min(1),
            ])
            .split(area);

        let popup_chunks_horizontal = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Max(left_right_distance),
                Constraint::Max(POPUP_WIDTH),
                Constraint::Min(1),
            ])
            .split(popup_chunks_vertical[1]);

        let popup_chunk = Rc::new(popup_chunks_horizontal[1]);

        Ok(Self {
            terminal_chunks,
            main_chunks,
            editor_chunks,
            popup_chunk,
        })
    }
}

#[cfg(test)]
mod tests {

    /// Need to test equality because we need equality to check if we need redrawing
    #[test]
    fn test_equality() {
        // ratatui
        use ratatui::layout::Rect;

        // Internal
        use super::Areas;

        let area_64 = Rect::new(0, 0, 64, 64);
        let area_128 = Rect::new(0, 0, 128, 128);

        let blocks_64_list = [
            Areas::split_area(area_64, 4usize).unwrap(),
            Areas::split_area(area_64, 4usize).unwrap(),
        ];

        let blocks_128_4 = Areas::split_area(area_128, 4usize).unwrap();
        let blocks_128_5 = Areas::split_area(area_128, 5usize).unwrap();

        assert_eq!(blocks_64_list[0], blocks_64_list[1]);
        assert_ne!(blocks_64_list[0], blocks_128_4);
        assert_ne!(blocks_128_4, blocks_128_5);
    }
}
