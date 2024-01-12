// Crate prelude

pub use crate::error::Error;

pub type Result<T> = core::result::Result<T, Error>;

pub use simplelog::{debug, error, info, trace, warn};
