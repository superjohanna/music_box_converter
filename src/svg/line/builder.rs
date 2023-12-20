use crate::vec2::Vec2;

use super::Line;

pub struct LineBuilder<T> {
    start: Option<Vec2<T>>,
    end: Option<Vec2<T>>,
    stroke: Option<String>,
    stroke_width: Option<T>,
}

impl<T> Line<T> {
    pub fn new_builder() -> LineBuilder<T> {
        LineBuilder {
            start: Option::None,
            end: Option::None,
            stroke: Option::None,
            stroke_width: Option::None,
        }
    }
}

impl<T> LineBuilder<T> {
    pub fn set_start(mut self, x: T, y: T) -> Self {
        self.start = Some(Vec2::new(x, y));
        self
    }
    pub fn set_end(mut self, x: T, y: T) -> Self {
        self.end = Some(Vec2::new(x, y));
        self
    }
    pub fn set_stroke(mut self, stroke: String) -> Self {
        self.stroke = Some(stroke);
        self
    }
    pub fn set_stroke_width(mut self, stroke_width: T) -> Self {
        self.stroke_width = Some(stroke_width);
        self
    }
    /// Panics if an option isn't set
    pub fn finish(mut self) -> Box<Line<T>> {
        Box::new(Line {
            start: self.start.unwrap(),
            end: self.end.unwrap(),
            stroke: self.stroke.unwrap(),
            stroke_width: self.stroke_width.unwrap(),
        })
    }
}
