// serde_derive
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Note {
    C(u8),
    CS(u8),
    D(u8),
    DS(u8),
    E(u8),
    F(u8),
    FS(u8),
    G(u8),
    GS(u8),
    A(u8),
    AS(u8),
    B(u8),
}
