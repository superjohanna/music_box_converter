// std
use std::default;

// ======= Settings =======
// Change this line if you want to change the settings struct used
type Settings = crate::settings::Settings;
// This is a prefix for items that belong to a group
const SUB_PREFIX: &str = "\u{2022}"; // • https://www.compart.com/en/unicode/U+2022

/// Just a ```Vec<SettingsGroup>```
pub type GroupList = Vec<SettingsGroup>;

/// A trait to implement Indexing for ```GroupList```
pub trait GroupListTrait {
    fn index_from_name(&self, name: String) -> Option<usize>;
    fn max_length(&self) -> Option<usize>;
    fn as_string(&self) -> String;
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

    fn as_string(&self) -> String {
        let mut s = String::new();
        for group in self {
            s.push_str(&group.name);
            for item in &group.items {
                s.push_str(&item.human_readable_name)
            }
        }
        s
    }
}

#[derive(Clone, Debug, Default)]
pub struct SettingsItem {
    pub name: String,
    pub human_readable_name: String,
    pub value_type: ValueType,
}

impl SettingsItem {
    pub fn new(name: String, human_readable_name: String, value_type: ValueType) -> Self {
        Self {
            name,
            human_readable_name,
            value_type,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum ValueType {
    #[default]
    Number,
    Colour,
}

pub fn get_groups() -> GroupList {
    let mut groups = GroupList::new();

    for (group, name, human_readable, value_type) in Settings::get_items() {
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
            .push(SettingsItem::new(name, human_readable, value_type));
    }

    groups
}
