// Crate prelude

pub use crate::error::Error;

pub type Result<T> = core::result::Result<T, Error>;

pub use simplelog::{debug, error, info, trace, warn};

pub trait ShortAsRef<T> {
    fn res(&self) -> Result<&T>;
    fn res_mut(&mut self) -> Result<&mut T>;
}

impl<T> ShortAsRef<T> for Option<T> {
    /// A Method as shortcut for as_ref().unwrap() which also doesn't panic
    fn res(&self) -> Result<&T> {
        match self {
            None => Err(Error::Internal("Something was executed in the wrong order. Please contact the author with information about what you did.".to_string())),
            Some(t) => Ok(self.as_ref().unwrap()),
        }
    }

    fn res_mut(&mut self) -> Result<&mut T> {
        match self {
            None => Err(Error::Internal("Something was executed in the wrong order. Please contact the author with information about what you did.".to_string())),
            Some(t) => Ok(self.as_mut().unwrap()),
        }
    }
}
