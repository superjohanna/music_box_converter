// Crate Error

#[derive(Debug, thiserror::Error, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Error {
    #[error("Generic {0}")]
    Generic(String),
}