// Crate Error

// std
use std::io;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Generic Error: {0}")]
    Generic(String),
    #[error("Internal Error: {0}")]
    Internal(String),
    #[error("{0}")]
    Displayable(String),

    #[error("")]
    TerminalTooSmall,

    #[error("IO Error: {0}")]
    Io(Box<io::Error>, Box<String>),
    #[error("Serialization Error: {0}")]
    SerdeJson(Box<serde_json::Error>),
    #[error("Midi Error: {0}")]
    Midi(Box<midly::Error>),
}
