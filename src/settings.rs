// serde_derive
use serde_derive::{Deserialize, Serialize};

// Internal
use crate::config_macro_add_item;
use crate::config_macro_list_items;
use crate::music_box_config::item_list::value::ValueType;
use crate::music_box_config::item_list::value::ValueWrapper;

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct Settings {
    // Notes
    pub note_hole_radius_mm: f64,
    pub note_hole_colour: String,

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
    pub sprocket_hole_enable: bool,
    pub sprocket_hole_distance_mm: f64,
    pub sprocket_hole_distance_staff_mm: f64,
    pub sprocket_hole_colour: String,
}

impl Settings {
    // Sets a value given an index
    pub fn set(&mut self, i: usize, val: &ValueWrapper) {
        match i {
            // Notes
            1 => self.note_hole_radius_mm = val.self_to_f64().unwrap(),
            2 => self.note_hole_colour = val.self_to_string().unwrap(),
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
            17 => self.sprocket_hole_enable = val.self_to_bool().unwrap(),
            18 => self.sprocket_hole_distance_mm = val.self_to_f64().unwrap(),
            19 => self.sprocket_hole_distance_staff_mm = val.self_to_f64().unwrap(),
            20 => self.sprocket_hole_colour = val.self_to_string().unwrap(),
            _ => (),
        }
    }

    // Gets a value given an index
    pub fn get(&self, i: usize) -> Option<ValueWrapper> {
        match i {
            // Notes
            1 => Some(ValueWrapper::from_f64(self.note_hole_radius_mm)),
            2 => Some(ValueWrapper::from_string(self.note_hole_colour.clone())),
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
            17 => Some(ValueWrapper::from_bool(self.sprocket_hole_enable)),
            18 => Some(ValueWrapper::from_f64(self.sprocket_hole_distance_mm)),
            19 => Some(ValueWrapper::from_f64(self.sprocket_hole_distance_staff_mm)),
            20 => Some(ValueWrapper::from_string(self.sprocket_hole_colour.clone())),
            _ => None,
        }
    }

    config_macro_list_items!(
        Settings,
        note_hole_radius_mm,
        note_hole_colour,
        staff_offset_mm,
        staff_line_thickness_mm,
        staff_line_colour,
        staff_bounding_box_thickness_mm,
        staff_bounding_box_top_bottom_distance_mm,
        staff_bounding_box_top_bottom_colour,
        staff_bounding_box_left_right_colour,
        paper_size_x,
        paper_size_y,
        sprocket_hole_enable,
        sprocket_hole_distance_mm,
        sprocket_hole_distance_staff_mm,
        sprocket_hole_colour,
    );

    // To add a new group with new items
    // Don't forget to add it to the top as well
    /*
    ui_macro_add_item!(
        "$Groupname"; // This is the name of the group
        $Fieldname,  // Name of the field you added above
        "$Readablename", // Human readable name
        $ValueTypeEnum,  // Is it a colour or a number or a bool etc.?
        $HelpString, // A lot of help strings are defined at the bottom
        // You can repeat the last four to add more items
    )
    */

    //Holes
    config_macro_add_item!(
        "Note holes",
        HELP_NOTE_GROUP;
        note_hole_radius_mm,
        "Note hole radius (mm)",
        ValueType::Number,
        HELP_NOTE_HOLE_RADIUS;
        note_hole_colour,
        "Note hole colour",
        ValueType::Colour,
        HELP_NOTE_HOLE_COLOUR;
    );

    // Staff general
    config_macro_add_item!(
        "Staff general",
        HELP_STAFF_GROUP;
        staff_offset_mm,
        "Staff offset (mm)",
        ValueType::Number,
        HELP_STAFF_OFFSET;
    );

    // Staff Lines
    config_macro_add_item!(
        "Staff Lines",
        HELP_STAFF_LINE_GROUP;
        staff_line_thickness_mm,
        "Staff line thickness (mm)",
        ValueType::Number,
        HELP_STAFF_LINE_THICKNESS;
        staff_line_colour,
        "Staff line colour",
        ValueType::Colour,
        HELP_STAFF_LINE_COLOUR;
    );

    // Bounding Box
    config_macro_add_item!(
        "Bounding box",
        HELP_BOUNDING_BOX_GROUP;
        staff_bounding_box_thickness_mm,
        "Staff bounding box thickness (mm)",
        ValueType::Number,
        HELP_BOUNDING_BOX_THICKNESS;
        staff_bounding_box_top_bottom_distance_mm,
        "Staff bounding box top/ bottom distance (mm)",
        ValueType::Number,
        HELP_BOUNDING_BOX_TOP_BOTTOM_DISTANCE;
        staff_bounding_box_top_bottom_colour,
        "Staff bounding box top/ bottom colour",
        ValueType::Colour,
        HELP_BOUNDING_BOX_TOP_BOTTOM_COLOUR;
        staff_bounding_box_left_right_colour,
        "Staff bounding box left/ right colour",
        ValueType::Colour,
        HELP_BOUNDING_BOX_LEFT_RIGHT_COLOUR;
    );

    // Paper size
    config_macro_add_item!(
        "Paper size",
        HELP_PAPER_SIZE_GROUP;
        paper_size_x,
        "Paper length (mm)",
        ValueType::Number,
        HELP_PAPER_LENGTH;
        paper_size_y,
        "Paper height (mm)",
        ValueType::Number,
        HELP_PAPER_HEIGHT;
    );

    // Sprocket holes
    config_macro_add_item!(
        "Sprocket holes",
        HELP_SPROCKET_GROUP;
        sprocket_hole_enable,
        "Sprocket holes enable",
        ValueType::Boolean,
        HELP_SPROCKET_ENABLE;
        sprocket_hole_distance_mm,
        "Sprocket hole distance (mm)",
        ValueType::Number,
        HELP_SPROCKET_HOLE_DISTANCE;
        sprocket_hole_distance_staff_mm,
        "Sprocket hole distance to staff (mm)",
        ValueType::Number,
        HELP_SPROCKET_HOLE_DISTANCE_STAFF;
        sprocket_hole_colour,
        "Sprocket holes colour",
        ValueType::Colour,
        HELP_SPROCKET_HOLE_COLOUR;
    );
}

// Help

// Notes
const HELP_NOTE_GROUP: &str =
    r#"These are settings concerning the note holes, the red circles in the example file."#;

const HELP_NOTE_HOLE_RADIUS: &str = r#"This is the radius of the note holes."#;

const HELP_NOTE_HOLE_COLOUR: &str = r#"This is the colour of the note holes."#;

// Staff
const HELP_STAFF_GROUP: &str = r#"These are general settings concerning the staff."#;

const HELP_STAFF_OFFSET: &str = r#"This is the offset between the top left corner of the file and the corner of the bounding box."#;

// Staff lines
const HELP_STAFF_LINE_GROUP: &str =
    r#"These are settings concerning the staff lines, the black lines in the example file."#;

const HELP_STAFF_LINE_THICKNESS: &str = r#"This is the thickness of the note lines."#;

const HELP_STAFF_LINE_COLOUR: &str = r#"This is the colour of the note lines."#;

// Bounding box
const HELP_BOUNDING_BOX_GROUP: &str = r#"These are settings concerning the bounding box, the green and magenta lines in the example file."#;

const HELP_BOUNDING_BOX_THICKNESS: &str = r#"This is the thickness of the bounding box lines."#;

const HELP_BOUNDING_BOX_TOP_BOTTOM_DISTANCE: &str =
    r#"This is the distance of the top and bottom bounding box lines to the staff."#;

const HELP_BOUNDING_BOX_TOP_BOTTOM_COLOUR: &str =
    r#"This is the colour of the top and bottom bounding box lines."#;

const HELP_BOUNDING_BOX_LEFT_RIGHT_COLOUR: &str =
    r#"This is the colour of the left and right bounding box lines."#;

// Paper size
const HELP_PAPER_SIZE_GROUP: &str = r#"These are settings concerning the paper sizes."#;

const HELP_PAPER_LENGTH: &str = r#"This is the length of the paper. If the length of the next note exceeds the paper length it will start a new file."#;

const HELP_PAPER_HEIGHT: &str = r#"This setting is checked against the music box strip height and if the strip height exceeds the paper size an error is returned."#;

// Sprocket holes
const HELP_SPROCKET_GROUP: &str =
    r#"These are settings concerning the sprocket holes, the yellow holes in the example file."#;

const HELP_SPROCKET_ENABLE: &str = r#"This enables or disables the sprocket holes."#;

const HELP_SPROCKET_HOLE_DISTANCE: &str =
    r#"This is the distance of the centres of two sprocket holes."#;

const HELP_SPROCKET_HOLE_DISTANCE_STAFF: &str = r#"This is the vertical distance of the sprocket holes to the staff (first staff line on the bottom and top, the black ones)."#;

const HELP_SPROCKET_HOLE_COLOUR: &str = r#"This is the colour of the sprocket holes."#;
