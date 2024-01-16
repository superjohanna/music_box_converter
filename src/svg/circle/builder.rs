use crate::vec2::Vec2;

use super::Circle;

pub struct CircleBuilder<T> {
    centre: Option<Vec2<T>>,
    radius: Option<T>,
    fill: Option<String>,
}

impl<T> Circle<T> {
    pub fn new_builder() -> CircleBuilder<T> {
        CircleBuilder {
            centre: Option::None,
            radius: Option::None,
            fill: Option::None,
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
    pub fn set_fill(mut self, fill: String) -> Self {
        self.fill = Some(fill);
        self
    }
    /// Panics if an option isn't set
    pub fn finish(mut self) -> Box<Circle<T>> {
        Box::new(Circle {
            centre: self.centre.unwrap(),
            radius: self.radius.unwrap(),
            fill: self.fill.unwrap(),
        })
    }
}
