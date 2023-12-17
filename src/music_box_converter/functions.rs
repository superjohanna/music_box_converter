// std
use std::{
    fs::{self, File},
    io::{BufReader, Read},
};

// midly
use midly::{Smf, Track, TrackEvent, TrackEventKind};

// serde
use serde::{Serialize, Serializer};
use svg::{Document, Node};

// Internal
use super::MusicBoxConverter;
use crate::{
    music::{meta_information::MetaInformation, music_box::MusicBox},
    prelude::*,
    settings::svg::SvgSettings,
    vec2::Vec2,
};

impl MusicBoxConverter {
    pub fn run(mut self) -> Result<()> {
        self.choose_music_box()?;
        self.load_svg_settings()?;
        // We can't make smf and bytes part of the MusicBoxConverter Struct because smf references bytes, so they need to have the same lifetime. ARGH!
        let bytes = self.get_bytes()?;
        let smf = Self::get_smf(&bytes)?;
        self.setup_document(&smf)?;
        self.output_document()?;

        Ok(())
    }

    /// Deserializes ./box.json and assigns the MusicBox with the name given via arguments to the self.music_box
    fn choose_music_box(&mut self) -> Result<()> {
        let file = match File::open(self.args.get_one::<String>("io_box").unwrap()) {
            Ok(t) => t,
            Err(e) => return Err(Error::IOError(Box::new(e))),
        };

        let deserialized: Vec<MusicBox> = match serde_json::from_reader(BufReader::new(file)) {
            Ok(t) => t,
            Err(e) => return Err(Error::SerdeJsonError(Box::new(e))),
        };

        let chosen_box: &String = match self.args.get_one("box") {
            // We provided a default value for this argument so if this is None something has seriously gone wrong
            None => {
                return Err(Error::Generic(
                    "music_box_converter/functions.rs Line 27".to_string(),
                ))
            }
            Some(t) => t,
        };

        for m in deserialized {
            if m.name != *chosen_box {
                continue;
            }
            if self.verbose {
                println!("Selected music box: {}", m.name)
            }
            self.music_box = Some(m);
            return Ok(());
        }

        Err(Error::Generic(
            format!("Music box '{0}' not found!", *chosen_box).to_string(),
        ))
    }

    /// Deserializes ./svg_settings.json and assigns the deserialized SvgSettings to self.svg_settings
    fn load_svg_settings(&mut self) -> Result<()> {
        let file = match File::open(self.args.get_one::<String>("io_svg_settings").unwrap()) {
            Ok(t) => t,
            Err(e) => return Err(Error::IOError(Box::new(e))),
        };

        let deserialized: SvgSettings = match serde_json::from_reader(BufReader::new(file)) {
            Ok(t) => t,
            Err(e) => return Err(Error::SerdeJsonError(Box::new(e))),
        };

        self.svg_settings = Some(deserialized);
        Ok(())
    }

    /// Returns a Vec<u8> with the data of the file
    fn get_bytes(&mut self) -> Result<Vec<u8>> {
        let mut arg = self.args.get_one::<String>("io_in").unwrap().to_owned();
        if arg.chars().collect::<Vec<char>>()[0] == ' ' {
            arg.remove(0);
        }

        let mut file: Vec<u8> = match std::fs::read(arg) {
            Ok(t) => t,
            Err(e) => return Err(Error::IOError(Box::new(e))),
        };

        Ok(file)
    }

    /// Returns a Smf which links to the bytes which were passed
    fn get_smf(bytes: &Vec<u8>) -> Result<Smf> {
        let smf: Smf = match Smf::parse(bytes) {
            Ok(t) => t,
            Err(e) => return Err(Error::MidiError(Box::new(e))),
        };

        Ok(smf)
    }

    /// Replaces the current document and adds the base layout
    fn setup_document(&mut self, smf: &Smf) -> Result<()> {
        let settings = self.svg_settings.clone().unwrap();
        self.meta = Option::Some(MetaInformation::gather_meta(&smf.tracks[0]));
        let mut scale_factor = Vec2::<f64>::new();
        // How much do we have to scale the notes for it to be compatible with the music box
        scale_factor.x = <u32 as Into<f64>>::into(self.meta.as_ref().unwrap().min_distance)
            / self.music_box.as_ref().unwrap().min_note_distance_mm;

        // How much space there is between two lines
        scale_factor.y = self.music_box.as_ref().unwrap().get_scale_factor_y();

        todo!();
        // Calculate how much fits onto one page
        // Place Notes into their own Vec<Vec<Note>>
        //                             |   |
        //                             |   Notes
        //                             Page
        // Then iterate over pages with the fill_document method (needs renaming) which iterates over notes and returns a document

        Ok(())
    }

    /// Fills the current document with the notes
    fn fill_document(&mut self, smf: &Smf) -> Result<()> {
        for track in smf.tracks.clone() {
            for track_event in track {
                match track_event.kind {
                    TrackEventKind::Midi { .. } => (),
                    _ => continue,
                }
            }
        }
        Ok(())
    }

    /// Outputs the current document to the specified output file
    fn output_document(&self) -> Result<()> {
        let path = std::path::Path::new(self.args.get_one::<String>("io_out").unwrap());

        let parent = match path.parent() {
            None => return Err(Error::Generic("Invalid output path".to_string())),
            Some(t) => t,
        };

        match fs::create_dir_all(path) {
            Ok(t) => (),
            Err(e) => return Err(Error::IOError(Box::new(e))),
        };

        let path_string = self.args.get_one::<String>("io_out").unwrap().to_owned();

        for (i, svg) in self.svg.iter().enumerate() {
            let mut path_string_i = path_string.clone();
            path_string_i.push('_');
            path_string_i.push_str(i.to_string().as_ref());
            path_string_i.push_str(".svg");

            match svg::save(path_string_i, svg) {
                Ok(t) => (),
                Err(e) => return Err(Error::IOError(Box::new(e))),
            }
        }
        Ok(())
    }
}
