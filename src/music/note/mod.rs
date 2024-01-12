// Modules
pub mod functions;

// serde_derive
use serde_derive::Deserialize;

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub enum Note {
    C(i8),
    CS(i8),
    D(i8),
    DS(i8),
    E(i8),
    F(i8),
    FS(i8),
    G(i8),
    GS(i8),
    A(i8),
    AS(i8),
    B(i8),
}
