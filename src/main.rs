#![allow(unused)]
// Modules
pub mod arguments;
pub mod error;
pub mod music;
pub mod music_box_converter;
pub mod prelude;
pub mod settings;
pub mod svg;
pub mod vec2;

// Internal
use crate::arguments::get_args;
use crate::music_box_converter::MusicBoxConverter;
use crate::prelude::*;

fn main() -> Result<()> {
    let args = get_args();
    let mut converter = MusicBoxConverter::new(args);
    converter.run()?;

    /* let thisdir = std::fs::canonicalize(std::path::PathBuf::from("./out/"));
    println!("{:?}", thisdir); */

    Ok(())
}
