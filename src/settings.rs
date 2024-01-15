// serde_derive
use serde_derive::Deserialize;

#[derive(Clone, Debug, Deserialize, Default)]
pub struct Settings {
    // Holes
    pub hole_radius_mm: f64,
    pub hole_colour: String,

    // Staff
    pub staff_offset_mm: f64,

    // Staff lines
    pub staff_line_thickness_mm: f64,
    pub staff_line_colour: String,

    // Bounding Box
    pub staff_bounding_box_thickness_mm: f64,
    pub staff_bounding_box_colour: String,
}
