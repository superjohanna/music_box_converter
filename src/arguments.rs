// clap
use clap::{Arg, ArgMatches, Command};

pub fn get_args() -> ArgMatches {
    Command::new("Music box converter")
        .about("Converts a MIDI file (*.midi|*.mid) into an SVG file (*.svg)")
        .version("0.1.0")
        .arg_required_else_help(true)
        .author("Johanna Wehner, superjohannaa@gmail.com")
        .arg(
            Arg::new("io_in")
                .short('i')
                .long("input")
                .help("Specifies the input file")
                .num_args(1)
                .value_name("FILE")
                .required(true),
        )
        .arg(
            Arg::new("io_out")
                .short('o')
                .long("output")
                .help("Specifies which folder to output to")
                .num_args(1)
                .value_name("FILE")
                .required(true),
        )
        .arg(
            Arg::new("io_box")
                .short('f')
                .long("boxes")
                .help("Specifies which music box file to use")
                .default_value("./box.json")
                .num_args(1)
                .value_name("FILE")
                .required(false),
        )
        .arg(
            Arg::new("io_svg_settings")
                .short('F')
                .long("svg-settings")
                .help("Specifies which svg settings file to use")
                .default_value("./svg_settings.json")
                .num_args(1)
                .value_name("FILE")
                .required(false),
        )
        .arg(
            Arg::new("box")
                .short('b')
                .long("box")
                .help("Specifies which music box from the box.json file to use")
                .default_value("30_note")
                .num_args(1)
                .value_name("FILE")
                .required(false),
        )
        .arg(
            Arg::new("verbosity")
                .short('v')
                .long("verbose")
                .help("Increases verbosity. Helpful for debugging")
                .default_value("false")
                .num_args(0)
                .value_name("verbose")
                .required(false),
        )
        .arg(
            Arg::new("force")
                .long("force")
                .help("Forces output in cwd. Not recommended due to amount of possible files")
                .default_value("false")
                .num_args(0)
                .value_name("force")
                .required(false),
        )
        .help_template(HELP_TEMPLATE)
        .get_matches()
}

const HELP_TEMPLATE: &str = r#"{name} (v{version}) by {author}

{about}

{usage-heading}    
{usage}

{all-args}
"#;
