pub mod builder;

use std::fmt::Pointer;

use crate::vec2::Vec2;

use super::document::Child;

#[derive(Debug, Clone)]
pub struct Line<T> {
    start: Vec2<T>,
    end: Vec2<T>,
    stroke: String,
    stroke_width: T,
}

impl<T: Clone + std::fmt::Display + 'static> Child for Line<T> {
    fn clone_dyn(&self) -> Box<dyn Child> {
        Box::new(self.clone())
    }

    fn fmt_dyn(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt(f)
    }

    fn print(&self, unit_suffix: String) -> String {
        let start_x = &self.start.x;
        let start_y = &self.start.y;
        let end_x = &self.end.x;
        let end_y = &self.end.y;
        let stroke = &self.stroke;
        let stroke_width = &self.stroke_width;
        format!(
            r#"<line x1="{start_x}{unit_suffix}" y1="{start_y}{unit_suffix}" x2="{end_x}{unit_suffix}" y2="{end_y}{unit_suffix}" stroke="{stroke}" stroke_width="{stroke_width}{unit_suffix}" />"#
        )
    }
}
