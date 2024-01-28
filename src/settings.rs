// serde_derive
use serde_derive::{Deserialize, Serialize};

// Internal
use crate::config_macro_add_item;
use crate::config_macro_list_items;
use crate::music_box_config::config_groups::ValueType;

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
}

impl Settings {
    // Sets a value given an index
    pub fn set(&mut self, i: usize, val: &StringOrF64) {
        match i {
            // Holes
            1 => self.hole_radius_mm = val.c_to_f64().unwrap(),
            2 => self.hole_colour = val.c_to_string().unwrap(),
            // Staff
            4 => self.staff_offset_mm = val.c_to_f64().unwrap(),
            // Staff lines
            6 => self.staff_line_thickness_mm = val.c_to_f64().unwrap(),
            7 => self.staff_line_colour = val.c_to_string().unwrap(),
            // Bounding box
            9 => self.staff_bounding_box_thickness_mm = val.c_to_f64().unwrap(),
            10 => self.staff_bounding_box_top_bottom_distance_mm = val.c_to_f64().unwrap(),
            11 => self.staff_bounding_box_top_bottom_colour = val.c_to_string().unwrap(),
            12 => self.staff_bounding_box_left_right_colour = val.c_to_string().unwrap(),
            // Paper sizes
            14 => self.paper_size_x = val.c_to_f64().unwrap(),
            15 => self.paper_size_y = val.c_to_f64().unwrap(),
            _ => (),
        }
    }

    // Gets a value given an index
    pub fn get(&self, i: usize) -> Option<StringOrF64> {
        match i {
            // Holes
            1 => Some(StringOrF64::from_f64(self.hole_radius_mm)),
            2 => Some(StringOrF64::from_string(self.hole_colour.clone())),
            // Staff
            4 => Some(StringOrF64::from_f64(self.staff_offset_mm)),
            // Staff lines
            6 => Some(StringOrF64::from_f64(self.staff_line_thickness_mm)),
            7 => Some(StringOrF64::from_string(self.staff_line_colour.clone())),
            // Bounding box
            9 => Some(StringOrF64::from_f64(self.staff_bounding_box_thickness_mm)),
            10 => Some(StringOrF64::from_f64(
                self.staff_bounding_box_top_bottom_distance_mm,
            )),
            11 => Some(StringOrF64::from_string(
                self.staff_bounding_box_top_bottom_colour.clone(),
            )),
            12 => Some(StringOrF64::from_string(
                self.staff_bounding_box_left_right_colour.clone(),
            )),
            // Paper sizes
            14 => Some(StringOrF64::from_f64(self.paper_size_x)),
            15 => Some(StringOrF64::from_f64(self.paper_size_y)),
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
        "Holes",
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
        "Staff general",
        staff_offset_mm,
        "Staff offset (mm)",
        ValueType::Number,
        f64,
        HELP_STAFF_OFFSET;
    );

    // Staff Lines
    config_macro_add_item!(
        self,
        "Staff Lines",
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
        "Bounding box",
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
        "Paper size",
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
}

#[derive(Debug, Clone)]
pub enum StringOrF64 {
    String(String),
    F64(f64),
}

impl StringOrF64 {
    pub fn c_to_string(&self) -> Option<String> {
        match self {
            Self::String(s) => Some(s.to_owned()),
            _ => None,
        }
    }

    pub fn c_to_f64(&self) -> Option<f64> {
        match self {
            Self::F64(f) => Some(*f),
            _ => None,
        }
    }

    pub fn from_string(s: String) -> StringOrF64 {
        Self::String(s)
    }

    pub fn from_f64(f: f64) -> StringOrF64 {
        Self::F64(f)
    }
}

impl ToString for StringOrF64 {
    fn to_string(&self) -> String {
        match self {
            Self::String(s) => s.to_owned(),
            Self::F64(f) => f.to_string(),
        }
    }
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
