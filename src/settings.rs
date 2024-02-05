// serde_derive
use serde_derive::{Deserialize, Serialize};

// Internal
use crate::config_macro_add_item;
use crate::config_macro_list_items;
use crate::music_box_config::config_groups::ValueType;
use crate::music_box_config::config_groups::ValueWrapper;

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

    // Bounding box
    pub staff_bounding_box_thickness_mm: f64,
    pub staff_bounding_box_top_bottom_distance_mm: f64,
    pub staff_bounding_box_top_bottom_colour: String,
    pub staff_bounding_box_left_right_colour: String,

    // Paper sizes
    pub paper_size_x: f64,
    pub paper_size_y: f64,

    // Sprocket holes
    pub sprocket_hole_distance: f64,
}

impl Settings {
    // Sets a value given an index
    pub fn set(&mut self, i: usize, val: &ValueWrapper) {
        match i {
            // Holes
            1 => self.hole_radius_mm = val.self_to_f64().unwrap(),
            2 => self.hole_colour = val.self_to_string().unwrap(),
            // Staff
            4 => self.staff_offset_mm = val.self_to_f64().unwrap(),
            // Staff lines
            6 => self.staff_line_thickness_mm = val.self_to_f64().unwrap(),
            7 => self.staff_line_colour = val.self_to_string().unwrap(),
            // Bounding box
            9 => self.staff_bounding_box_thickness_mm = val.self_to_f64().unwrap(),
            10 => self.staff_bounding_box_top_bottom_distance_mm = val.self_to_f64().unwrap(),
            11 => self.staff_bounding_box_top_bottom_colour = val.self_to_string().unwrap(),
            12 => self.staff_bounding_box_left_right_colour = val.self_to_string().unwrap(),
            // Paper sizes
            14 => self.paper_size_x = val.self_to_f64().unwrap(),
            15 => self.paper_size_y = val.self_to_f64().unwrap(),
            // Sprocket holes
            17 => self.sprocket_hole_distance = val.self_to_f64().unwrap(),
            _ => (),
        }
    }

    // Gets a value given an index
    pub fn get(&self, i: usize) -> Option<ValueWrapper> {
        match i {
            // Holes
            1 => Some(ValueWrapper::from_f64(self.hole_radius_mm)),
            2 => Some(ValueWrapper::from_string(self.hole_colour.clone())),
            // Staff
            4 => Some(ValueWrapper::from_f64(self.staff_offset_mm)),
            // Staff lines
            6 => Some(ValueWrapper::from_f64(self.staff_line_thickness_mm)),
            7 => Some(ValueWrapper::from_string(self.staff_line_colour.clone())),
            // Bounding box
            9 => Some(ValueWrapper::from_f64(self.staff_bounding_box_thickness_mm)),
            10 => Some(ValueWrapper::from_f64(
                self.staff_bounding_box_top_bottom_distance_mm,
            )),
            11 => Some(ValueWrapper::from_string(
                self.staff_bounding_box_top_bottom_colour.clone(),
            )),
            12 => Some(ValueWrapper::from_string(
                self.staff_bounding_box_left_right_colour.clone(),
            )),
            // Paper sizes
            14 => Some(ValueWrapper::from_f64(self.paper_size_x)),
            15 => Some(ValueWrapper::from_f64(self.paper_size_y)),
            // Sprocket holes
            17 => Some(ValueWrapper::from_f64(self.sprocket_hole_distance)),
            _ => None,
        }
    }

    config_macro_list_items!(
        Settings,
        hole_radius_mm,
        hole_colour,
        staff_offset_mm,
        staff_line_thickness_mm,
        staff_line_colour,
        staff_bounding_box_thickness_mm,
        staff_bounding_box_top_bottom_distance_mm,
        staff_bounding_box_top_bottom_colour,
        staff_bounding_box_left_right_colour,
        paper_size_x,
        paper_size_y,
        sprocket_hole_distance,
    );

    // To add a new group with new items
    // Don't forget to add it to the top as well
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
    config_macro_add_item!(
        self,
        "Holes";
        hole_radius_mm,
        "Hole radius (mm)",
        ValueType::Number,
        f64,
        HELP_HOLE_RADIUS;
        hole_colour,
        "Hole colour",
        ValueType::Colour,
        String,
        HELP_HOLE_COLOUR;
    );

    // Staff general
    config_macro_add_item!(
        self,
        "Staff general";
        staff_offset_mm,
        "Staff offset (mm)",
        ValueType::Number,
        f64,
        HELP_STAFF_OFFSET;
    );

    // Staff Lines
    config_macro_add_item!(
        self,
        "Staff Lines";
        staff_line_thickness_mm,
        "Staff line thickness (mm)",
        ValueType::Number,
        f64,
        HELP_STAFF_LINE_THICKNESS;
        staff_line_colour,
        "Staff line colour",
        ValueType::Colour,
        String,
        HELP_STAFF_LINE_COLOUR;
    );

    // Bounding Box
    config_macro_add_item!(
        self,
        "Bounding box";
        staff_bounding_box_thickness_mm,
        "Staff bounding box thickness (mm)",
        ValueType::Number,
        f64,
        HELP_BOUNDING_BOX_THICKNESS;
        staff_bounding_box_top_bottom_distance_mm,
        "Staff bounding box top/ bottom distance (mm)",
        ValueType::Number,
        f64,
        HELP_BOUNDING_BOX_TOP_BOTTOM_DISTANCE;
        staff_bounding_box_top_bottom_colour,
        "Staff bounding box top/ bottom colour",
        ValueType::Colour,
        String,
        HELP_BOUNDING_BOX_TOP_BOTTOM_COLOUR;
        staff_bounding_box_left_right_colour,
        "Staff bounding box left/ right colour",
        ValueType::Colour,
        String,
        HELP_BOUNDING_BOX_LEFT_RIGHT_COLOUR;
    );

    // Paper size
    config_macro_add_item!(
        self,
        "Paper size";
        paper_size_x,
        "Paper length (mm)",
        ValueType::Number,
        f64,
        HELP_PAPER_LENGTH;
        paper_size_y,
        "Paper height (mm)",
        ValueType::Number,
        f64,
        HELP_PAPER_HEIGHT;
    );

    // Sprocket holes
    config_macro_add_item!(
        self,
        "Sprocket holes";
        sprocket_hole_distance,
        "Sprocket hole distance (mm)",
        ValueType::Number,
        f64,
        HELP_SPROCKET_HOLE_DISTANCE;
    );
}

// Help

// Holes
const HELP_HOLE_RADIUS: &str =
    r#"This is the radius of the holes. The red circles in the example file."#;

const HELP_HOLE_COLOUR: &str =
    r#"This is the colour of the holes. The red circles in the example file."#;

// Staff
const HELP_STAFF_OFFSET: &str = r#"This is the offset between the top left corner of the file and the corner of the bounding box."#;

// Staff lines
const HELP_STAFF_LINE_THICKNESS: &str =
    r#"This is the thickness of the note lines. They are black in the example file. "#;

const HELP_STAFF_LINE_COLOUR: &str =
    r#"This is the colour of the note lines. They are black in the example file."#;

// Bounding box
const HELP_BOUNDING_BOX_THICKNESS: &str =
    r#"This is the thickness of the bounding box. They are green and magenta in the example file."#;

const HELP_BOUNDING_BOX_TOP_BOTTOM_DISTANCE: &str = r#"This is the distance of the top and bottom bounding box lines to the staff. They are green in the example file."#;

const HELP_BOUNDING_BOX_TOP_BOTTOM_COLOUR: &str = r#"This is the colour of the top and bottom bounding box lines. In the example file they are green."#;

const HELP_BOUNDING_BOX_LEFT_RIGHT_COLOUR: &str = r#"This is the colour of the left and right bounding box lines. In the example file they are magenta."#;

// Paper size
const HELP_PAPER_LENGTH: &str = r#"This is the length of the paper. If the length of the next note exceeds the paper length it will start a new file."#;

const HELP_PAPER_HEIGHT: &str = r#"This setting is currently unused."#;

// Sprocket holes
const HELP_SPROCKET_HOLE_DISTANCE: &str = r#"This is the distance of the centres of two sprocket holes. In the example file they are also red, but they are one the edges in fixed distances."#;
