// std
use std::default;

use serde_json::Value;

// ======= Settings =======
// Change this line if you want to change the settings struct used
type Settings = crate::settings::Settings;
// This is a prefix for items that belong to a group
const SUB_PREFIX: &str = "\u{2022}"; // • https://www.compart.com/en/unicode/U+2022

/// Just a `Vec<SettingsGroup>`
pub type GroupList = Vec<SettingsGroup>;

/// A trait to implement Indexing for `GroupList`
pub trait GroupListTrait {
    /// Finds the index from a name. Returns`None` if the item is not found.
    fn index_from_name(&self, name: String) -> Option<usize>;
    /// Finds the length of the longest human readable name of the items
    fn max_length(&self) -> Option<usize>;
    /// Returns a `Vec<bool>`, which contains as many members as there are groups and items,
    /// and which denote if the item with this index is a group or an item
    fn get_list_value_type_and_help(&self) -> Vec<(ValueType, String)>;
}

#[derive(Clone, Debug, Default)]
pub struct SettingsGroup {
    /// The name of the group
    pub name: String,
    /// The length of the longest item human readable name
    pub max_length: usize,
    /// The items that belong to the group
    pub items: Vec<SettingsItem>,
}

impl SettingsGroup {
    pub fn new(name: String) -> Self {
        Self {
            name,
            ..Default::default()
        }
    }
}

impl GroupListTrait for GroupList {
    fn index_from_name(&self, name: String) -> Option<usize> {
        self.iter().position(|x| x.name == name)
    }

    fn max_length(&self) -> Option<usize> {
        self.iter()
            .max_by_key(|x| x.max_length)
            .map(|x| x.max_length)
    }

    fn get_list_value_type_and_help(&self) -> Vec<(ValueType, String)> {
        let mut v = Vec::<(ValueType, String)>::new();
        for group in self {
            v.push((ValueType::None, String::new()));
            for item in group.items.iter() {
                v.push((item.value_type, item.help.clone()));
            }
        }
        v
    }
}

#[derive(Clone, Debug, Default)]
pub struct SettingsItem {
    pub name: String,
    pub human_readable_name: String,
    pub value_type: ValueType,
    pub help: String,
}

impl SettingsItem {
    pub fn new(
        name: String,
        human_readable_name: String,
        value_type: ValueType,
        help: String,
    ) -> Self {
        Self {
            name,
            human_readable_name,
            value_type,
            help,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub enum ValueType {
    /// This means it is a group instead of an item
    #[default]
    None,
    /// The value is a number
    Number,
    /// The value is a colour
    Colour,
    /// The value is a bool
    Boolean,
}

impl ValueType {
    pub fn is_none(&self) -> bool {
        matches!(self, ValueType::None)
    }
}

/// Used to wrap different types so we only have one type
#[derive(Debug, Clone)]
pub enum ValueWrapper {
    String(String),
    F64(f64),
    Boolean(bool),
}

impl ValueWrapper {
    pub fn self_to_string(&self) -> Option<String> {
        match self {
            Self::String(s) => Some(s.to_owned()),
            _ => None,
        }
    }

    pub fn self_to_f64(&self) -> Option<f64> {
        match self {
            Self::F64(f) => Some(*f),
            _ => None,
        }
    }

    pub fn self_to_bool(&self) -> Option<bool> {
        match self {
            Self::Boolean(b) => Some(*b),
            _ => None,
        }
    }

    pub fn from_string(s: String) -> ValueWrapper {
        Self::String(s)
    }

    pub fn from_f64(f: f64) -> ValueWrapper {
        Self::F64(f)
    }

    pub fn from_bool(b: bool) -> ValueWrapper {
        Self::Boolean(b)
    }
}

impl ToString for ValueWrapper {
    fn to_string(&self) -> String {
        match self {
            Self::String(s) => s.to_owned(),
            Self::F64(f) => f.to_string(),
            Self::Boolean(b) => b.to_string(),
        }
    }
}

pub fn get_groups() -> GroupList {
    let mut groups = GroupList::new();

    for (group, name, human_readable, value_type, help) in Settings::get_items() {
        let i = match groups.index_from_name(group.clone()) {
            Some(t) => t,
            None => {
                groups.push(SettingsGroup::new(group.clone()));
                groups.len() - 1
            }
        };

        let human_readable = SUB_PREFIX.to_owned() + &human_readable;

        if human_readable.len() > groups[i].max_length {
            groups[i].max_length = human_readable.len();
        }

        groups[i]
            .items
            .push(SettingsItem::new(name, human_readable, value_type, help));
    }

    groups
}
