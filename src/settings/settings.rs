// serde_derive
use serde_derive::Deserialize;

#[derive(Clone, Debug, Deserialize, Default)]
pub struct Settings {
    // Sizes
    pub staff_offset_mm: f64,
    pub hole_radius_mm: f64,

    // Thickness
    pub staff_line_thickness_mm: f64,

    // Colour
    pub staff_bounding_box_colour: String,
    pub staff_line_colour: String,
    pub hole_colour: String,

    // Midi
    pub note_min_delay_ticks: u64,
}
