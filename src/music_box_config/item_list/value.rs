#[derive(Debug, Clone, Copy, PartialEq, Default)]
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
#[derive(Debug, Clone, PartialEq)]
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

impl std::fmt::Display for ValueWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(s) => write!(f, "{}", s),
            Self::F64(fl) => write!(f, "{}", fl),
            Self::Boolean(b) => write!(f, "{}", b),
        }
    }
}
