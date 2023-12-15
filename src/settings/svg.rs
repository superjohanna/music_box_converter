// serde_derive
use serde_derive::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct SvgSettings {
    // Sizes
    pub staff_offset: u32,
    pub hole_radius: u32,

    // Colour
    pub staff_bounding_box_colour: Option<String>,
    pub staff_line_colour: Option<String>,
    pub hole_colour: Option<String>,
}

impl SvgSettings {
    pub fn new() -> Self {
        Self {
            staff_offset: 0,
            hole_radius: 0,

            staff_bounding_box_colour: None,
            staff_line_colour: None,
            hole_colour: None,
        }
    }
}
