// serde_derive
use serde_derive::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct SvgSettings {
    // Sizes
    pub staff_offset_mm: f64,
    pub hole_radius_mm: f64,

    // Thickness
    pub staff_line_thickness_mm: f64,

    // Colour
    pub staff_bounding_box_colour: String,
    pub staff_line_colour: String,
    pub hole_colour: String,
}

impl SvgSettings {
    pub fn new() -> Self {
        Self {
            staff_offset_mm: Default::default(),
            hole_radius_mm: Default::default(),

            staff_line_thickness_mm: Default::default(),

            staff_bounding_box_colour: Default::default(),
            staff_line_colour: Default::default(),
            hole_colour: Default::default(),
        }
    }
}
