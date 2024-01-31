// clap
use clap::{value_parser, Arg, ArgAction, ArgMatches, Command};

pub fn get_args() -> Command {
    Command::new("music_box_converter")
        .subcommand(
            Command::new("convert")
                .about("Converts a MIDI file (*.midi|*.mid) into an SVG file (*.svg)")
                .version("1.0.0")
                .arg_required_else_help(true)
                .author("Johanna Wehner, superjohannaa@gmail.com")
                .arg(
                    Arg::new("io_in")
                        .short('i')
                        .long("input")
                        .help("The input file to use.")
                        .num_args(1)
                        .value_name("FILE")
                        .required(true),
                )
                .arg(
                    Arg::new("io_out")
                        .short('o')
                        .long("output")
                        .help("The output directory to output to.")
                        .num_args(1)
                        .value_name("DIRECTORY")
                        .required(true),
                )
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
                .arg(
                    Arg::new("io_box")
                        .help("Specifies which box file to use.")
                        .default_value("./box.json")
                        .num_args(1)
                        .value_name("FILE")
                        .required(false),
                )
                .arg(
                    Arg::new("io_out_midi")
                        .short('O')
                        .long("midi-out")
                        .help("When set outputs a midi file with transposed notes on one track and the original track.")
                        .num_args(1)
                        .value_name("FILE")
                        .required(false),
                )
                .arg(
                    Arg::new("track")
                        .short('T')
                        .long("track")
                        .help("Specifies which track from the midi file to use. Zero-based.")
                        .default_value("0")
                        .value_parser(value_parser!(usize))
                        .num_args(1)
                        .value_name("TRACK_NUMBER")
                        .required(false),
                )
                .arg(
                    Arg::new("transpose")
                        .short('t')
                        .long("transpose")
                        .help("Wether to transpose notes that can't normally be played.")
                        .default_value("false")
                        .num_args(0)
                        .required(false),
                )
                .arg(
                    Arg::new("verbosity")
                        .short('v')
                        .long("verbose")
                        .help("Increases verbosity. Can be used multiple times to raise log level.")
                        .default_value("0")
                        .num_args(0)
                        .action(ArgAction::Count)
                        .required(false)
                        .conflicts_with("quiet"),
                )
                .arg(
                    Arg::new("quiet")
                        .short('q')
                        .long("quiet")
                        .help("No Output. Exclusive to verbosity")
                        .default_value("false")
                        .num_args(0)
                        .conflicts_with("verbosity"),
                )
                .arg(
                    Arg::new("force")
                        .long("force")
                        .help("Allows to output into the current working directory.")
                        .default_value("false")
                        .num_args(0)
                        .required(false),
                )
                .help_template(HELP_TEMPLATE),
        )
        .subcommand(
            Command::new("config")
                .about("GUI configuration program for the converter")
                .version("1.0.0")
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
                .help_template(HELP_TEMPLATE),
        )
}

const HELP_TEMPLATE: &str = r#"{name} (v{version}) by {author}

{about}

{usage-heading}    
{usage}

{all-args}
"#;
