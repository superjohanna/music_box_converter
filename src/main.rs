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
            Err(e) => Err(Error::IOError(
                Box::new(e),
                Box::new("Error printing help".to_string()),
            )),
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

#[cfg(test)]
mod tests {
    use crate::music_box_convert::MusicBoxConvert;

    fn run(args: clap::ArgMatches) -> Vec<String> {
        match match args.subcommand() {
            Some(("convert", sub_m)) => music_box_convert(sub_m),
            _ => panic!("Invalid subcommand"),
        } {
            Ok(t) => t,
            Err(_) => panic!(""),
        }
    }

    fn music_box_convert(args: &clap::ArgMatches) -> crate::prelude::Result<Vec<String>> {
        let mut converter = MusicBoxConvert::new(args);
        converter.run_output_string()
    }

    #[test]
    fn test_default() {
        let args = [
            "program_name",
            "convert",
            "-i",
            "meg_wiwauf_laminat1.mid",
            "-o",
            "./out/",
            "-qt",
        ];

        let command = crate::arguments::get_args();
        let res = run(command.get_matches_from(args));

        assert_eq!(
            res[0],
            r##"<svg version="1.1" xmlns="http://www.w3.org/2000/svg">
<line x1="10mm" y1="10mm" x2="306mm" y2="10mm" stroke="#000000" stroke_width="1mm" />
<line x1="10mm" y1="12.006551724137932mm" x2="306mm" y2="12.006551724137932mm" stroke="#000000" stroke_width="1mm" />
<line x1="10mm" y1="14.013103448275862mm" x2="306mm" y2="14.013103448275862mm" stroke="#000000" stroke_width="1mm" />
<line x1="10mm" y1="16.019655172413792mm" x2="306mm" y2="16.019655172413792mm" stroke="#000000" stroke_width="1mm" />
<line x1="10mm" y1="18.026206896551724mm" x2="306mm" y2="18.026206896551724mm" stroke="#000000" stroke_width="1mm" />
<line x1="10mm" y1="20.032758620689656mm" x2="306mm" y2="20.032758620689656mm" stroke="#000000" stroke_width="1mm" />
<line x1="10mm" y1="22.039310344827584mm" x2="306mm" y2="22.039310344827584mm" stroke="#000000" stroke_width="1mm" />
<line x1="10mm" y1="24.045862068965516mm" x2="306mm" y2="24.045862068965516mm" stroke="#000000" stroke_width="1mm" />
<line x1="10mm" y1="26.052413793103447mm" x2="306mm" y2="26.052413793103447mm" stroke="#000000" stroke_width="1mm" />
<line x1="10mm" y1="28.05896551724138mm" x2="306mm" y2="28.05896551724138mm" stroke="#000000" stroke_width="1mm" />
<line x1="10mm" y1="30.06551724137931mm" x2="306mm" y2="30.06551724137931mm" stroke="#000000" stroke_width="1mm" />
<line x1="10mm" y1="32.07206896551724mm" x2="306mm" y2="32.07206896551724mm" stroke="#000000" stroke_width="1mm" />
<line x1="10mm" y1="34.07862068965517mm" x2="306mm" y2="34.07862068965517mm" stroke="#000000" stroke_width="1mm" />
<line x1="10mm" y1="36.0851724137931mm" x2="306mm" y2="36.0851724137931mm" stroke="#000000" stroke_width="1mm" />
<line x1="10mm" y1="38.09172413793103mm" x2="306mm" y2="38.09172413793103mm" stroke="#000000" stroke_width="1mm" />
<line x1="10mm" y1="40.09827586206896mm" x2="306mm" y2="40.09827586206896mm" stroke="#000000" stroke_width="1mm" />
<line x1="10mm" y1="42.104827586206895mm" x2="306mm" y2="42.104827586206895mm" stroke="#000000" stroke_width="1mm" />
<line x1="10mm" y1="44.11137931034482mm" x2="306mm" y2="44.11137931034482mm" stroke="#000000" stroke_width="1mm" />
<line x1="10mm" y1="46.11793103448276mm" x2="306mm" y2="46.11793103448276mm" stroke="#000000" stroke_width="1mm" />
<line x1="10mm" y1="48.12448275862069mm" x2="306mm" y2="48.12448275862069mm" stroke="#000000" stroke_width="1mm" />
<line x1="10mm" y1="50.13103448275862mm" x2="306mm" y2="50.13103448275862mm" stroke="#000000" stroke_width="1mm" />
<line x1="10mm" y1="52.13758620689655mm" x2="306mm" y2="52.13758620689655mm" stroke="#000000" stroke_width="1mm" />
<line x1="10mm" y1="54.14413793103448mm" x2="306mm" y2="54.14413793103448mm" stroke="#000000" stroke_width="1mm" />
<line x1="10mm" y1="56.150689655172414mm" x2="306mm" y2="56.150689655172414mm" stroke="#000000" stroke_width="1mm" />
<line x1="10mm" y1="58.15724137931034mm" x2="306mm" y2="58.15724137931034mm" stroke="#000000" stroke_width="1mm" />
<line x1="10mm" y1="60.16379310344827mm" x2="306mm" y2="60.16379310344827mm" stroke="#000000" stroke_width="1mm" />
<line x1="10mm" y1="62.170344827586206mm" x2="306mm" y2="62.170344827586206mm" stroke="#000000" stroke_width="1mm" />
<line x1="10mm" y1="64.17689655172413mm" x2="306mm" y2="64.17689655172413mm" stroke="#000000" stroke_width="1mm" />
<line x1="10mm" y1="66.18344827586206mm" x2="306mm" y2="66.18344827586206mm" stroke="#000000" stroke_width="1mm" />
<line x1="10mm" y1="68.19mm" x2="306mm" y2="68.19mm" stroke="#000000" stroke_width="1mm" />
<line x1="10mm" y1="5mm" x2="10mm" y2="73.19mm" stroke="#ff00ff" stroke_width="1mm" />
<line x1="306mm" y1="5mm" x2="306mm" y2="73.19mm" stroke="#ff00ff" stroke_width="1mm" />
<line x1="10mm" y1="5mm" x2="306mm" y2="5mm" stroke="#00ff00" stroke_width="1mm" />
<line x1="10mm" y1="73.19mm" x2="306mm" y2="73.19mm" stroke="#00ff00" stroke_width="1mm" />
<circle cx="10mm" cy="54.14413793103448mm" r="1mm" fill="#ff0000" />
<circle cx="10mm" cy="62.170344827586206mm" r="1mm" fill="#ff0000" />
<circle cx="10mm" cy="58.15724137931034mm" r="1mm" fill="#ff0000" />
<circle cx="10mm" cy="54.14413793103448mm" r="1mm" fill="#ff0000" />
<circle cx="18mm" cy="54.14413793103448mm" r="1mm" fill="#ff0000" />
<circle cx="26mm" cy="54.14413793103448mm" r="1mm" fill="#ff0000" />
<circle cx="26mm" cy="64.17689655172413mm" r="1mm" fill="#ff0000" />
<circle cx="26mm" cy="60.16379310344827mm" r="1mm" fill="#ff0000" />
<circle cx="26mm" cy="54.14413793103448mm" r="1mm" fill="#ff0000" />
<circle cx="34mm" cy="58.15724137931034mm" r="1mm" fill="#ff0000" />
<circle cx="34mm" cy="62.170344827586206mm" r="1mm" fill="#ff0000" />
<circle cx="34mm" cy="58.15724137931034mm" r="1mm" fill="#ff0000" />
<circle cx="34mm" cy="52.13758620689655mm" r="1mm" fill="#ff0000" />
<circle cx="42mm" cy="60.16379310344827mm" r="1mm" fill="#ff0000" />
<circle cx="42mm" cy="62.170344827586206mm" r="1mm" fill="#ff0000" />
<circle cx="42mm" cy="56.150689655172414mm" r="1mm" fill="#ff0000" />
<circle cx="42mm" cy="52.13758620689655mm" r="1mm" fill="#ff0000" />
<circle cx="50mm" cy="60.16379310344827mm" r="1mm" fill="#ff0000" />
<circle cx="50mm" cy="56.150689655172414mm" r="1mm" fill="#ff0000" />
<circle cx="50mm" cy="54.14413793103448mm" r="1mm" fill="#ff0000" />
<circle cx="58mm" cy="62.170344827586206mm" r="1mm" fill="#ff0000" />
<circle cx="58mm" cy="62.170344827586206mm" r="1mm" fill="#ff0000" />
<circle cx="58mm" cy="58.15724137931034mm" r="1mm" fill="#ff0000" />
<circle cx="58mm" cy="54.14413793103448mm" r="1mm" fill="#ff0000" />
<circle cx="74mm" cy="62.170344827586206mm" r="1mm" fill="#ff0000" />
<circle cx="74mm" cy="58.15724137931034mm" r="1mm" fill="#ff0000" />
<circle cx="74mm" cy="52.13758620689655mm" r="1mm" fill="#ff0000" />
<circle cx="82mm" cy="62.170344827586206mm" r="1mm" fill="#ff0000" />
<circle cx="86mm" cy="62.170344827586206mm" r="1mm" fill="#ff0000" />
<circle cx="90mm" cy="60.16379310344827mm" r="1mm" fill="#ff0000" />
<circle cx="98mm" cy="58.15724137931034mm" r="1mm" fill="#ff0000" />
<circle cx="102mm" cy="58.15724137931034mm" r="1mm" fill="#ff0000" />
<circle cx="106mm" cy="64.17689655172413mm" r="1mm" fill="#ff0000" />
<circle cx="106mm" cy="58.15724137931034mm" r="1mm" fill="#ff0000" />
<circle cx="106mm" cy="54.14413793103448mm" r="1mm" fill="#ff0000" />
<circle cx="114mm" cy="58.15724137931034mm" r="1mm" fill="#ff0000" />
<circle cx="118mm" cy="58.15724137931034mm" r="1mm" fill="#ff0000" />
<circle cx="122mm" cy="56.150689655172414mm" r="1mm" fill="#ff0000" />
<circle cx="122mm" cy="64.17689655172413mm" r="1mm" fill="#ff0000" />
<circle cx="122mm" cy="60.16379310344827mm" r="1mm" fill="#ff0000" />
<circle cx="122mm" cy="56.150689655172414mm" r="1mm" fill="#ff0000" />
<circle cx="130mm" cy="54.14413793103448mm" r="1mm" fill="#ff0000" />
<circle cx="134mm" cy="54.14413793103448mm" r="1mm" fill="#ff0000" />
<circle cx="138mm" cy="64.17689655172413mm" r="1mm" fill="#ff0000" />
<circle cx="138mm" cy="58.15724137931034mm" r="1mm" fill="#ff0000" />
<circle cx="138mm" cy="54.14413793103448mm" r="1mm" fill="#ff0000" />
<circle cx="154mm" cy="60.16379310344827mm" r="1mm" fill="#ff0000" />
<circle cx="154mm" cy="56.150689655172414mm" r="1mm" fill="#ff0000" />
<circle cx="154mm" cy="54.14413793103448mm" r="1mm" fill="#ff0000" />
<circle cx="170mm" cy="44.11137931034482mm" r="1mm" fill="#ff0000" />
<circle cx="170mm" cy="62.170344827586206mm" r="1mm" fill="#ff0000" />
<circle cx="170mm" cy="58.15724137931034mm" r="1mm" fill="#ff0000" />
<circle cx="170mm" cy="54.14413793103448mm" r="1mm" fill="#ff0000" />
<circle cx="178mm" cy="44.11137931034482mm" r="1mm" fill="#ff0000" />
<circle cx="186mm" cy="44.11137931034482mm" r="1mm" fill="#ff0000" />
<circle cx="186mm" cy="62.170344827586206mm" r="1mm" fill="#ff0000" />
<circle cx="186mm" cy="60.16379310344827mm" r="1mm" fill="#ff0000" />
<circle cx="186mm" cy="50.13103448275862mm" r="1mm" fill="#ff0000" />
<circle cx="194mm" cy="48.12448275862069mm" r="1mm" fill="#ff0000" />
<circle cx="202mm" cy="54.14413793103448mm" r="1mm" fill="#ff0000" />
<circle cx="202mm" cy="64.17689655172413mm" r="1mm" fill="#ff0000" />
<circle cx="202mm" cy="58.15724137931034mm" r="1mm" fill="#ff0000" />
<circle cx="202mm" cy="54.14413793103448mm" r="1mm" fill="#ff0000" />
<circle cx="218mm" cy="56.150689655172414mm" r="1mm" fill="#ff0000" />
<circle cx="218mm" cy="62.170344827586206mm" r="1mm" fill="#ff0000" />
<circle cx="218mm" cy="56.150689655172414mm" r="1mm" fill="#ff0000" />
<circle cx="218mm" cy="50.13103448275862mm" r="1mm" fill="#ff0000" />
<circle cx="234mm" cy="58.15724137931034mm" r="1mm" fill="#ff0000" />
<circle cx="234mm" cy="62.170344827586206mm" r="1mm" fill="#ff0000" />
<circle cx="234mm" cy="58.15724137931034mm" r="1mm" fill="#ff0000" />
<circle cx="234mm" cy="52.13758620689655mm" r="1mm" fill="#ff0000" />
<circle cx="242mm" cy="56.150689655172414mm" r="1mm" fill="#ff0000" />
<circle cx="250mm" cy="60.16379310344827mm" r="1mm" fill="#ff0000" />
<circle cx="250mm" cy="64.17689655172413mm" r="1mm" fill="#ff0000" />
<circle cx="250mm" cy="60.16379310344827mm" r="1mm" fill="#ff0000" />
<circle cx="250mm" cy="56.150689655172414mm" r="1mm" fill="#ff0000" />
<circle cx="258mm" cy="60.16379310344827mm" r="1mm" fill="#ff0000" />
<circle cx="266mm" cy="62.170344827586206mm" r="1mm" fill="#ff0000" />
<circle cx="266mm" cy="62.170344827586206mm" r="1mm" fill="#ff0000" />
<circle cx="266mm" cy="58.15724137931034mm" r="1mm" fill="#ff0000" />
<circle cx="266mm" cy="54.14413793103448mm" r="1mm" fill="#ff0000" />
<circle cx="298mm" cy="44.11137931034482mm" r="1mm" fill="#ff0000" />
<circle cx="298mm" cy="62.170344827586206mm" r="1mm" fill="#ff0000" />
<circle cx="298mm" cy="58.15724137931034mm" r="1mm" fill="#ff0000" />
<circle cx="298mm" cy="54.14413793103448mm" r="1mm" fill="#ff0000" />
<circle cx="306mm" cy="44.11137931034482mm" r="1mm" fill="#ff0000" />
</svg>"##.to_string()
        );

        assert_eq!(
            res[1],
            r##"<svg version="1.1" xmlns="http://www.w3.org/2000/svg">
<line x1="10mm" y1="10mm" x2="98mm" y2="10mm" stroke="#000000" stroke_width="1mm" />
<line x1="10mm" y1="12.006551724137932mm" x2="98mm" y2="12.006551724137932mm" stroke="#000000" stroke_width="1mm" />
<line x1="10mm" y1="14.013103448275862mm" x2="98mm" y2="14.013103448275862mm" stroke="#000000" stroke_width="1mm" />
<line x1="10mm" y1="16.019655172413792mm" x2="98mm" y2="16.019655172413792mm" stroke="#000000" stroke_width="1mm" />
<line x1="10mm" y1="18.026206896551724mm" x2="98mm" y2="18.026206896551724mm" stroke="#000000" stroke_width="1mm" />
<line x1="10mm" y1="20.032758620689656mm" x2="98mm" y2="20.032758620689656mm" stroke="#000000" stroke_width="1mm" />
<line x1="10mm" y1="22.039310344827584mm" x2="98mm" y2="22.039310344827584mm" stroke="#000000" stroke_width="1mm" />
<line x1="10mm" y1="24.045862068965516mm" x2="98mm" y2="24.045862068965516mm" stroke="#000000" stroke_width="1mm" />
<line x1="10mm" y1="26.052413793103447mm" x2="98mm" y2="26.052413793103447mm" stroke="#000000" stroke_width="1mm" />
<line x1="10mm" y1="28.05896551724138mm" x2="98mm" y2="28.05896551724138mm" stroke="#000000" stroke_width="1mm" />
<line x1="10mm" y1="30.06551724137931mm" x2="98mm" y2="30.06551724137931mm" stroke="#000000" stroke_width="1mm" />
<line x1="10mm" y1="32.07206896551724mm" x2="98mm" y2="32.07206896551724mm" stroke="#000000" stroke_width="1mm" />
<line x1="10mm" y1="34.07862068965517mm" x2="98mm" y2="34.07862068965517mm" stroke="#000000" stroke_width="1mm" />
<line x1="10mm" y1="36.0851724137931mm" x2="98mm" y2="36.0851724137931mm" stroke="#000000" stroke_width="1mm" />
<line x1="10mm" y1="38.09172413793103mm" x2="98mm" y2="38.09172413793103mm" stroke="#000000" stroke_width="1mm" />
<line x1="10mm" y1="40.09827586206896mm" x2="98mm" y2="40.09827586206896mm" stroke="#000000" stroke_width="1mm" />
<line x1="10mm" y1="42.104827586206895mm" x2="98mm" y2="42.104827586206895mm" stroke="#000000" stroke_width="1mm" />
<line x1="10mm" y1="44.11137931034482mm" x2="98mm" y2="44.11137931034482mm" stroke="#000000" stroke_width="1mm" />
<line x1="10mm" y1="46.11793103448276mm" x2="98mm" y2="46.11793103448276mm" stroke="#000000" stroke_width="1mm" />
<line x1="10mm" y1="48.12448275862069mm" x2="98mm" y2="48.12448275862069mm" stroke="#000000" stroke_width="1mm" />
<line x1="10mm" y1="50.13103448275862mm" x2="98mm" y2="50.13103448275862mm" stroke="#000000" stroke_width="1mm" />
<line x1="10mm" y1="52.13758620689655mm" x2="98mm" y2="52.13758620689655mm" stroke="#000000" stroke_width="1mm" />
<line x1="10mm" y1="54.14413793103448mm" x2="98mm" y2="54.14413793103448mm" stroke="#000000" stroke_width="1mm" />
<line x1="10mm" y1="56.150689655172414mm" x2="98mm" y2="56.150689655172414mm" stroke="#000000" stroke_width="1mm" />
<line x1="10mm" y1="58.15724137931034mm" x2="98mm" y2="58.15724137931034mm" stroke="#000000" stroke_width="1mm" />
<line x1="10mm" y1="60.16379310344827mm" x2="98mm" y2="60.16379310344827mm" stroke="#000000" stroke_width="1mm" />
<line x1="10mm" y1="62.170344827586206mm" x2="98mm" y2="62.170344827586206mm" stroke="#000000" stroke_width="1mm" />
<line x1="10mm" y1="64.17689655172413mm" x2="98mm" y2="64.17689655172413mm" stroke="#000000" stroke_width="1mm" />
<line x1="10mm" y1="66.18344827586206mm" x2="98mm" y2="66.18344827586206mm" stroke="#000000" stroke_width="1mm" />
<line x1="10mm" y1="68.19mm" x2="98mm" y2="68.19mm" stroke="#000000" stroke_width="1mm" />
<line x1="10mm" y1="5mm" x2="10mm" y2="73.19mm" stroke="#ff00ff" stroke_width="1mm" />
<line x1="98mm" y1="5mm" x2="98mm" y2="73.19mm" stroke="#ff00ff" stroke_width="1mm" />
<line x1="10mm" y1="5mm" x2="98mm" y2="5mm" stroke="#00ff00" stroke_width="1mm" />
<line x1="10mm" y1="73.19mm" x2="98mm" y2="73.19mm" stroke="#00ff00" stroke_width="1mm" />
<circle cx="18mm" cy="44.11137931034482mm" r="1mm" fill="#ff0000" />
<circle cx="18mm" cy="62.170344827586206mm" r="1mm" fill="#ff0000" />
<circle cx="18mm" cy="60.16379310344827mm" r="1mm" fill="#ff0000" />
<circle cx="18mm" cy="50.13103448275862mm" r="1mm" fill="#ff0000" />
<circle cx="26mm" cy="48.12448275862069mm" r="1mm" fill="#ff0000" />
<circle cx="34mm" cy="54.14413793103448mm" r="1mm" fill="#ff0000" />
<circle cx="34mm" cy="64.17689655172413mm" r="1mm" fill="#ff0000" />
<circle cx="34mm" cy="58.15724137931034mm" r="1mm" fill="#ff0000" />
<circle cx="34mm" cy="54.14413793103448mm" r="1mm" fill="#ff0000" />
<circle cx="50mm" cy="56.150689655172414mm" r="1mm" fill="#ff0000" />
<circle cx="50mm" cy="62.170344827586206mm" r="1mm" fill="#ff0000" />
<circle cx="50mm" cy="56.150689655172414mm" r="1mm" fill="#ff0000" />
<circle cx="50mm" cy="50.13103448275862mm" r="1mm" fill="#ff0000" />
<circle cx="66mm" cy="58.15724137931034mm" r="1mm" fill="#ff0000" />
<circle cx="66mm" cy="62.170344827586206mm" r="1mm" fill="#ff0000" />
<circle cx="66mm" cy="58.15724137931034mm" r="1mm" fill="#ff0000" />
<circle cx="66mm" cy="52.13758620689655mm" r="1mm" fill="#ff0000" />
<circle cx="74mm" cy="56.150689655172414mm" r="1mm" fill="#ff0000" />
<circle cx="82mm" cy="60.16379310344827mm" r="1mm" fill="#ff0000" />
<circle cx="82mm" cy="64.17689655172413mm" r="1mm" fill="#ff0000" />
<circle cx="82mm" cy="60.16379310344827mm" r="1mm" fill="#ff0000" />
<circle cx="82mm" cy="56.150689655172414mm" r="1mm" fill="#ff0000" />
<circle cx="90mm" cy="60.16379310344827mm" r="1mm" fill="#ff0000" />
<circle cx="98mm" cy="62.170344827586206mm" r="1mm" fill="#ff0000" />
<circle cx="98mm" cy="62.170344827586206mm" r="1mm" fill="#ff0000" />
<circle cx="98mm" cy="58.15724137931034mm" r="1mm" fill="#ff0000" />
<circle cx="98mm" cy="54.14413793103448mm" r="1mm" fill="#ff0000" />
</svg>"##.to_string()
        );
    }
    
}
