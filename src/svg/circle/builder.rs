use crate::vec2::Vec2;

use super::Circle;

pub struct CircleBuilder<T> {
    centre: Option<Vec2<T>>,
    radius: Option<T>,
    stroke: Option<String>,
    stroke_width: Option<T>,
}

impl<T> Circle<T> {
    pub fn new_builder() -> CircleBuilder<T> {
        CircleBuilder {
            centre: Option::None,
            radius: Option::None,
            stroke: Option::None,
            stroke_width: Option::None,
        }
    }
}

impl<T> CircleBuilder<T> {
    pub fn set_centre(mut self, x: T, y: T) -> Self {
        self.centre = Some(Vec2::new(x, y));
        self
    }
    pub fn set_radius(mut self, x: T) -> Self {
        self.radius = Some(x);
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
    pub fn finish(mut self) -> Box<Circle<T>> {
        Box::new(Circle {
            centre: self.centre.unwrap(),
            radius: self.radius.unwrap(),
            stroke: self.stroke.unwrap(),
            stroke_width: self.stroke_width.unwrap(),
        })
    }
}
