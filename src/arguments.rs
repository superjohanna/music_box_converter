// clap
use clap::{value_parser, Arg, ArgAction, ArgMatches, Command};

pub fn get_args() -> Command {
    Command::new("music_box_converter")
        .subcommand(Command::new("convert")
            .about("Converts a MIDI file (*.midi|*.mid) into an SVG file (*.svg)")
            .version("0.1.0")
            .arg_required_else_help(true)
            .author("Johanna Wehner, superjohannaa@gmail.com")
            .arg(
                Arg::new("io_in")
                    .short('i')
                    .long("input")
                    .help("Specifies the input file.")
                    .num_args(1)
                    .value_name("FILE")
                    .required(true),
            )
            .arg(
                Arg::new("io_out")
                    .short('o')
                    .long("output")
                    .help("Specifies which folder to output to.")
                    .num_args(1)
                    .value_name("FILE")
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
                Arg::new("box")
                    .short('b')
                    .long("box")
                    .help("Specifies which music box from the boxes.json file to use.")
                    .default_value("30_note")
                    .num_args(1)
                    .value_name("FILE")
                    .required(false),
            )
            .arg(
                Arg::new("track")
                    .short('T')
                    .long("track")
                    .help("Which track of the input midi file to use. Zero-based.")
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
                    .help("Transposes notes which can't be played in their octave to the nearest one which can be played.")
                    .default_value("false")
                    .num_args(0)
                    .required(false),
            )
            .arg(
                Arg::new("verbosity")
                    .short('v')
                    .long("verbose")
                    .help("Increases verbosity. Can be used multiple times to raise log level.")
                    .default_value("false")
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
                    .help("Forces output in cwd. Not recommended due to amount of possible files.")
                    .default_value("false")
                    .num_args(0)
                    .required(false),
            )
            .help_template(HELP_TEMPLATE)
        )
        .subcommand(Command::new("config")
            .about("GUI configuration program for the converter")
            .version("0.1.0")
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
            .help_template(HELP_TEMPLATE)
        )
}

const HELP_TEMPLATE: &str = r#"{name} (v{version}) by {author}

{about}

{usage-heading}    
{usage}

{all-args}
"#;
