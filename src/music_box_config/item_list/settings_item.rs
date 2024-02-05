// Internal
use super::value::ValueType;

#[derive(Debug, Clone, Default)]
pub struct SettingsItem {
    /// The name of the item.
    pub name: String,
    /// The human readable name of the item.
    pub human_name: String,
    /// The value of the item. See [super::ValueType] for more.
    pub value_type: ValueType,
    /// The help page of the item
    pub help: String,
}

impl SettingsItem {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn new_group(name: String) -> Self {
        Self {
            name: name.clone(),
            human_name: name,
            ..Default::default()
        }
    }
}
