// serde_derive
use serde_derive::{Deserialize, Serialize};

// Internal
use crate::music_box_config::ui::ValueType;
use crate::ui_macro_add_item;
use crate::ui_macro_list_items;

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
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
    pub staff_bounding_box_top_bottom_distance_mm: f64,
    pub staff_bounding_box_top_bottom_colour: String,
    pub staff_bounding_box_left_right_colour: String,
}

impl Settings {
    ui_macro_list_items!(
        Settings,
        hole_radius_mm,
        hole_colour,
        staff_offset_mm,
        staff_line_thickness_mm,
        staff_line_colour,
        staff_bounding_box_thickness_mm,
        staff_bounding_box_top_bottom_distance_mm,
        staff_bounding_box_top_bottom_colour,
        staff_bounding_box_left_right_colour
    );

    //Holes
    ui_macro_add_item!(
        self,
        "Holes",
        hole_radius_mm,
        ValueType::Number,
        f64,
        hole_colour,
        ValueType::Colour,
        String
    );

    // Staff general
    ui_macro_add_item!(
        self,
        "Staff general",
        staff_offset_mm,
        ValueType::Number,
        f64
    );

    // Staff Lines
    ui_macro_add_item!(
        self,
        "Staff Lines",
        staff_line_thickness_mm,
        ValueType::Number,
        f64,
        staff_line_colour,
        ValueType::Colour,
        String
    );

    // Bounding Box
    ui_macro_add_item!(
        self,
        "Bounding Box",
        staff_bounding_box_thickness_mm,
        ValueType::Number,
        f64,
        staff_bounding_box_top_bottom_distance_mm,
        ValueType::Number,
        f64,
        staff_bounding_box_top_bottom_colour,
        ValueType::Colour,
        String,
        staff_bounding_box_left_right_colour,
        ValueType::Colour,
        String
    );
}
