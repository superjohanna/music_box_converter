pub mod builder;

use std::fmt::Pointer;

use crate::vec2::Vec2;

use super::document::Child;

#[derive(Debug, Clone)]
pub struct Circle<T> {
    centre: Vec2<T>,
    radius: T,
    fill: String,
}

impl<T: Clone + std::fmt::Display + 'static> Child for Circle<T> {
    fn clone_dyn(&self) -> Box<dyn Child> {
        Box::new(self.clone())
    }

    fn fmt_dyn(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt(f)
    }

    fn print(&self, unit_suffix: String) -> String {
        let centre_x = &self.centre.x;
        let centre_y = &self.centre.y;
        let radius = &self.radius;
        let fill = &self.fill;
        format!(
            r#"<circle cx="{centre_x}{unit_suffix}" cy="{centre_y}{unit_suffix}" r="{radius}{unit_suffix}" fill="{fill}" />"#
        )
    }
}

impl<T> Circle<T> {
    pub fn new(centre: Vec2<T>, radius: T, fill: String, stroke_width: T) -> Self {
        Self {
            centre,
            radius,
            fill,
        }
    }
}
