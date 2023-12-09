// Modules
pub mod arguments;
pub mod error;
pub mod music_box_converter;
pub mod prelude;

// Internal
use crate::arguments::get_args;
use crate::music_box_converter::MusicBoxConverter;
use crate::prelude::*;

fn main() -> Result<()> {
    let args = get_args();
    let converter = MusicBoxConverter::new(args);
    converter.run()?;
    Ok(())
}
