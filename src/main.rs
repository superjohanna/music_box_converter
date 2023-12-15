#![allow(unused)]
// Modules
pub mod arguments;
pub mod error;
pub mod music;
pub mod music_box_converter;
pub mod prelude;
pub mod settings;

// Internal
use crate::arguments::get_args;
use crate::music_box_converter::MusicBoxConverter;
use crate::prelude::*;

fn main() -> Result<()> {
    let args = get_args();
    let mut converter = MusicBoxConverter::new(args);
    converter.run()?;
    Ok(())
}
