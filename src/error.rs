// Crate Error

// std
use std::io;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Generic {0}")]
    Generic(String),
    #[error("IO Error {0}")]
    IOError(Box<io::Error>),
    #[error("Serialization Error {0}")]
    SerdeJsonError(Box<serde_json::Error>),
    #[error("Midi Error {0}")]
    MidiError(Box<midly::Error>),
}
