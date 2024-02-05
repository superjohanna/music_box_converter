// Internal
use super::{settings_item::SettingsItem, value::ValueType, Settings, SUB_PREFIX};

#[derive(Debug, Clone, Default)]
pub struct SettingsItemList {
    /// The items of the list
    pub items: Vec<SettingsItem>,
    /// The the length of the longest human readable name of the items
    pub longest_human_readable_name_length: usize,
}

impl SettingsItemList {
    pub fn new() -> Self {
        Self::default()
    }

    /// Gets a list of items that represent all the settings
    pub fn get_items() -> Self {
        let mut items = SettingsItemList::new();

        // Loop over all elements
        for (group, name, human_name, value_type, help) in Settings::get_items() {
            if !items.iter().any(|x| x.name == group) {
                // Add a new group if it doesn't exist
                items.push(SettingsItem::new_group(group));
            }

            // Shadowing doesn't work for some reason
            /* if value_type != ValueType::None {
                let human_name = SUB_PREFIX.to_owned() + &human_name;
            } */

            // Add a prefix if it is not a group and update length if it is longer
            let human_name_n = if value_type != ValueType::None {
                if human_name.len() + 2 > items.longest_human_readable_name_length {
                    items.longest_human_readable_name_length = human_name.len() + 2;
                }
                Some(SUB_PREFIX.to_owned() + &human_name)
            } else {
                if human_name.len() > items.longest_human_readable_name_length {
                    items.longest_human_readable_name_length = human_name.len();
                }
                None
            };

            // Add item
            if let Some(t) = human_name_n {
                items.push(SettingsItem {
                    name,
                    human_name: t,
                    value_type,
                    help,
                });
                continue;
            }

            items.push(SettingsItem {
                name,
                human_name,
                value_type,
                help,
            });
        }

        items
    }
}

impl std::ops::Deref for SettingsItemList {
    type Target = Vec<SettingsItem>;

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl std::ops::DerefMut for SettingsItemList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.items
    }
}

impl<Idx: std::slice::SliceIndex<[SettingsItem], Output = SettingsItem>> std::ops::Index<Idx>
    for SettingsItemList
{
    type Output = SettingsItem;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.items[index]
    }
}

impl<Idx: std::slice::SliceIndex<[SettingsItem], Output = SettingsItem>> std::ops::IndexMut<Idx>
    for SettingsItemList
{
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        &mut self.items[index]
    }
}
