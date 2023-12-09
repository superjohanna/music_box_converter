// CLAP
use clap::{Arg, ArgMatches, Command};

pub fn get_args() -> ArgMatches {
    Command::new("Music box converter")
        .about("Converts a MIDI file (*.midi|*.mid) into an SVG file (*.svg)")
        .version("0.1.0")
        .arg_required_else_help(true)
        .author("Johanna Wehner, superjohannaa@gmail.com")
        .arg(
            Arg::new("input file")
                .short('i')
                .long("input")
                .help("Specifies the input file")
                .num_args(1)
                .value_name("FILE")
                .required(true),
        )
        .arg(
            Arg::new("output file")
                .short('o')
                .long("output")
                .help("Specifies the output file")
                .num_args(1)
                .value_name("FILE")
                .required(true),
        )
        .arg(
            Arg::new("verbosity")
                .short('v')
                .long("verbose")
                .help("Increases verbosity. Helpful for debugging")
                .num_args(0)
                .value_name("verbose")
                .required(false),
        )
        .arg(
            Arg::new("music box file")
                .short('b')
                .long("box")
                .help("Specifies which music box file to use")
                .default_value("./boxes/default.box")
                .num_args(1)
                .value_name("FILE")
                .required(false),
        )
        .get_matches()
}
