// clap
use clap::{Arg, Command};

// Internal
use super::MusicBoxConfig;

impl MusicBoxConfig {
    pub fn get_command() -> Command {
        Command::new("config")
            .about("GUI configuration program for the converter")
            .version("1.0.1")
            .arg_required_else_help(false)
            .author("Johanna Wehner, superjohannaa@gmail.com")
            .arg(
                Arg::new("io_settings")
                    .short('s')
                    .long("settings")
                    .help("Specifies which settings file to use.")
                    .default_value("./settings.json")
                    .num_args(1)
                    .value_name("FILE")
                    .required(false),
            )
            .help_template(crate::command::HELP_TEMPLATE)
    }
}
