// Crate prelude

pub use crate::error::Error;

pub type Result<T> = core::result::Result<T, Error>;

pub use simplelog::{debug, error, info, trace, warn};

pub trait FromError<T> {
    fn from_io(res: std::io::Result<T>) -> Result<T>;
    fn from_serde_json(res: serde_json::Result<T>) -> Result<T>;
    fn from_midi(res: midly::Result<T>) -> Result<T>;
}

impl<T> FromError<T> for Result<T> {
    fn from_io(res: std::io::Result<T>) -> Result<T> {
        match res {
            Ok(t) => Ok(t),
            Err(e) => Err(Error::IOError(
                Box::new(e),
                Box::new("This io error was converted".to_string()),
            )),
        }
    }

    fn from_serde_json(res: serde_json::Result<T>) -> Result<T> {
        match res {
            Ok(t) => Ok(t),
            Err(e) => Err(Error::SerdeJsonError(Box::new(e))),
        }
    }

    fn from_midi(res: midly::Result<T>) -> Result<T> {
        match res {
            Ok(t) => Ok(t),
            Err(e) => Err(Error::MidiError(Box::new(e))),
        }
    }
}

pub trait FromIoError<T> {
    fn to_res(self) -> Result<T>;
}

impl<T> FromIoError<T> for std::io::Result<T> {
    fn to_res(self) -> Result<T> {
        Result::from_io(self)
    }
}

pub trait FromSerdeJson<T> {
    fn to_res(self) -> Result<T>;
}

impl<T> FromSerdeJson<T> for serde_json::Result<T> {
    fn to_res(self) -> Result<T> {
        Result::from_serde_json(self)
    }
}

pub trait FromMidi<T> {
    fn to_res(self) -> Result<T>;
}

impl<T> FromMidi<T> for midly::Result<T> {
    fn to_res(self) -> Result<T> {
        Result::from_midi(self)
    }
}

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
