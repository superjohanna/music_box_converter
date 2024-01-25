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

    // Bounding Box
    pub staff_bounding_box_thickness_mm: f64,
    pub staff_bounding_box_top_bottom_distance_mm: f64,
    pub staff_bounding_box_top_bottom_colour: String,
    pub staff_bounding_box_left_right_colour: String,
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
            // Bounding Box
            9 => self.staff_bounding_box_thickness_mm = val.c_to_f64().unwrap(),
            10 => self.staff_bounding_box_top_bottom_distance_mm = val.c_to_f64().unwrap(),
            11 => self.staff_bounding_box_top_bottom_colour = val.c_to_string().unwrap(),
            12 => self.staff_bounding_box_left_right_colour = val.c_to_string().unwrap(),
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
            // Bounding Box
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
        hole_colour,
        "Hole colour",
        ValueType::Colour,
        String,
    );

    // Staff general
    config_macro_add_item!(
        self,
        "Staff general",
        staff_offset_mm,
        "Staff offset (mm)",
        ValueType::Number,
        f64,
    );

    // Staff Lines
    config_macro_add_item!(
        self,
        "Staff Lines",
        staff_line_thickness_mm,
        "Staff line thickness (mm)",
        ValueType::Number,
        f64,
        staff_line_colour,
        "Staff line colour",
        ValueType::Colour,
        String,
    );

    // Bounding Box
    config_macro_add_item!(
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
        String,
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
