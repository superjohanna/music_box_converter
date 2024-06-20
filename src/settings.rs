use std::{
    collections::{btree_map::Range, HashMap},
    default,
};

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SettingsMap(HashMap<String, SettingsItem>);

impl std::ops::Deref for SettingsMap {
    type Target = HashMap<String, SettingsItem>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for SettingsMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub fn get_key_list() -> Vec<String> {
    [""].into_iter().map(|x| x.to_string()).collect()
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SettingsItem {
    pub value: ValueWrapper,
    pub translation_help_index: String,
    pub translation_name_index: String,
}

impl SettingsItem {
    pub fn value_type(&self) -> ValueType {
        match self.value {
            ValueWrapper::None => ValueType::None,
            ValueWrapper::String(_) => ValueType::String,
            ValueWrapper::F64(_) => ValueType::F64,
            ValueWrapper::Boolean(_) => ValueType::Boolean,
        }
    }
}

impl std::ops::Deref for SettingsItem {
    type Target = ValueWrapper;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::ops::DerefMut for SettingsItem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub enum ValueWrapper {
    #[default]
    None,
    String(String),
    F64(f64),
    Boolean(bool),
}

impl ValueWrapper {
    pub fn as_string(&self) -> Option<&String> {
        match self {
            Self::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<&f64> {
        match self {
            Self::F64(f) => Some(f),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<&bool> {
        match self {
            Self::Boolean(b) => Some(b),
            _ => None,
        }
    }
}

impl std::fmt::Display for ValueWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ValueWrapper::None => return Ok(()),
                ValueWrapper::String(s) => s.to_owned(),
                ValueWrapper::F64(f) => f.to_string(),
                ValueWrapper::Boolean(b) => b.to_string(),
            }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ValueType {
    #[default]
    None,
    String,
    F64,
    Boolean,
}

impl ValueType {
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    pub fn is_string(&self) -> bool {
        matches!(self, Self::String)
    }

    pub fn is_f64(&self) -> bool {
        matches!(self, Self::F64)
    }

    pub fn is_bool(&self) -> bool {
        matches!(self, Self::Boolean)
    }
}

impl std::fmt::Display for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ValueType::None => "ValueType::None",
                ValueType::String => "ValueType::String",
                ValueType::F64 => "ValueType::F64",
                ValueType::Boolean => "ValueType::Boolean",
            }
        )
    }
}
