// Mod
pub mod settings_item;
pub mod settings_item_list;
pub mod value;

// std
use std::{default, ops::Deref};

use serde::de;
// serde_json
use serde_json::Value;

// Internal
use self::{settings_item::SettingsItem, settings_item_list::SettingsItemList};

// ======= Settings =======
// Change this line if you want to change the settings struct used
type Settings = crate::settings::Settings;
// This is a prefix for items that belong to a group
const SUB_PREFIX: &str = "\u{2022} "; // • https://www.compart.com/en/unicode/U+2022
