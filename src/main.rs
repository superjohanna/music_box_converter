#![allow(unused)]
// Modules
pub mod arguments;
pub mod error;
pub mod music;
pub mod music_box_config;
pub mod music_box_convert;
pub mod path;
pub mod prelude;
pub mod settings;
pub mod svg;
pub mod vec2;

use music_box_config::MusicBoxConfig;

// Internal
use crate::arguments::get_args;
use crate::music_box_convert::MusicBoxConvert;
use crate::prelude::*;

fn main() -> Result<()> {
    let args = get_args().get_matches();
    let result = match args.subcommand() {
        Some(("convert", sub_m)) => music_box_convert(sub_m),
        Some(("config", sub_m)) => music_box_config(sub_m),
        _ => match get_args().print_help() {
            Ok(t) => Ok(t),
            Err(e) => Err(Error::IOError(Box::new(e))),
        },
    };

    match result {
        Ok(t) => (),
        Err(e) => error!("{:?}", e),
    }

    Ok(())
}

fn music_box_config(args: &clap::ArgMatches) -> Result<()> {
    let mut config = MusicBoxConfig::new(args);
    config.run()
}

fn music_box_convert(args: &clap::ArgMatches) -> Result<()> {
    let mut converter = MusicBoxConvert::new(args);
    converter.run_output_file()
}

mod tests {

    #[cfg(test)]
    fn test_1() {}
}
