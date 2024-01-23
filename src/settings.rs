// serde_derive
use serde_derive::{Deserialize, Serialize};

// Internal
use crate::music_box_config::ui::ui_groups::ValueType;
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

    // To add a new group with new items
    /*
    ui_macro_add_item!(
        self, // This needs to always be self
        "$Groupname" // This is the name of the group
        $Fieldname  // Name of the field you added above
        "$Readablename" // Human readable name
        $ValueTypeEnum  // Is it a colour or a number?
        $ValueType  // For example i16
        // You can repeat the last three to add more items
    )
    */

    //Holes
    ui_macro_add_item!(
        self,
        "Holes",
        hole_radius_mm,
        "Hole radius (mm)",
        ValueType::Number,
        f64,
        hole_colour,
        "Hole colour",
        ValueType::Colour,
        String
    );

    // Staff general
    ui_macro_add_item!(
        self,
        "Staff general",
        staff_offset_mm,
        "Staff offset (mm)",
        ValueType::Number,
        f64
    );

    // Staff Lines
    ui_macro_add_item!(
        self,
        "Staff Lines",
        staff_line_thickness_mm,
        "Staff line thickness (mm)",
        ValueType::Number,
        f64,
        staff_line_colour,
        "Staff line colour",
        ValueType::Colour,
        String
    );

    // Bounding Box
    ui_macro_add_item!(
        self,
        "Bounding Box",
        staff_bounding_box_thickness_mm,
        "Staff bounding box thickness (mm)",
        ValueType::Number,
        f64,
        staff_bounding_box_top_bottom_distance_mm,
        "Staff bounding box top/ bottom distance (mm)",
        ValueType::Number,
        f64,
        staff_bounding_box_top_bottom_colour,
        "Staff bounding box top/ bottom colour",
        ValueType::Colour,
        String,
        staff_bounding_box_left_right_colour,
        "Staff bounding box left/ right colour",
        ValueType::Colour,
        String
    );
}
