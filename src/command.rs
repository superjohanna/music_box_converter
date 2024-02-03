// clap
use clap::{value_parser, Arg, ArgAction, ArgMatches, Command};

pub fn get_command() -> Command {
    Command::new("music_box_converter")
        .subcommand(crate::music_box_convert::MusicBoxConvert::get_command())
        .subcommand(crate::music_box_config::MusicBoxConfig::get_command())
}

/// The Help template used by all sub applications
pub const HELP_TEMPLATE: &str = r#"{name} (v{version}) by {author}

{about}

{usage-heading}    
{usage}

{all-args}
"#;
