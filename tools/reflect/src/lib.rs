use std::{error::Error, fmt::Display};

pub trait Reflect {
    fn set(&mut self, index: usize, value: ValueWrapper);
    fn get(&self, index: usize) -> ValueWrapper;
    fn field_name(index: usize) -> &'static str;
    fn field_type(index: usize) -> ValueType;
}

#[derive(Debug, Clone, PartialEq)]
pub enum ValueWrapper {
    Unknown,
    String(String),
    F64(f64),
    Bool(bool),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValueWrapperWrongVariantError;

impl Display for ValueWrapperWrongVariantError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Wrong Enum variant!")
    }
}

impl Error for ValueWrapperWrongVariantError {}

impl TryFrom<ValueWrapper> for String {
    type Error = ValueWrapperWrongVariantError;

    fn try_from(value: ValueWrapper) -> Result<Self, Self::Error> {
        match value {
            ValueWrapper::String(string) => Ok(string),
            _ => Err(ValueWrapperWrongVariantError),
        }
    }
}

impl TryFrom<ValueWrapper> for f64 {
    type Error = ValueWrapperWrongVariantError;

    fn try_from(value: ValueWrapper) -> Result<Self, Self::Error> {
        match value {
            ValueWrapper::F64(float) => Ok(float),
            _ => Err(ValueWrapperWrongVariantError),
        }
    }
}

impl TryFrom<ValueWrapper> for bool {
    type Error = ValueWrapperWrongVariantError;

    fn try_from(value: ValueWrapper) -> Result<Self, Self::Error> {
        match value {
            ValueWrapper::Bool(bool) => Ok(bool),
            _ => Err(ValueWrapperWrongVariantError),
        }
    }
}

impl From<String> for ValueWrapper {
    fn from(value: String) -> ValueWrapper {
        ValueWrapper::String(value)
    }
}

impl From<f64> for ValueWrapper {
    fn from(value: f64) -> ValueWrapper {
        ValueWrapper::F64(value)
    }
}

impl From<bool> for ValueWrapper {
    fn from(value: bool) -> ValueWrapper {
        ValueWrapper::Bool(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValueType {
    Unknown,
    String,
    F64,
    Bool,
}
