// std
use std::{
    fs::File,
    io::{BufReader, Read},
};

// midly
use midly::{Smf, Track, TrackEvent, TrackEventKind};

// serde
use serde::{Serialize, Serializer};
use svg::{Document, Node};

// Internal
use super::MusicBoxConverter;
use crate::{music::music_box::MusicBox, prelude::*, settings::svg::SvgSettings};

impl MusicBoxConverter {
    pub fn run(mut self) -> Result<()> {
        self.choose_music_box()?;
        self.load_svg_settings()?;
        let bytes = self.get_bytes()?;
        let smf = Self::get_smf(&bytes)?;
        self.setup_document()?;
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

    /// Returns a Vec<u8> with the data from the midi file
    fn get_bytes(&self) -> Result<Vec<u8>> {
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
    fn get_smf<'a>(bytes: &'a [u8]) -> Result<Smf<'a>> {
        let smf = match Smf::parse(bytes) {
            Ok(t) => t,
            Err(e) => return Err(Error::MidiError(Box::new(e))),
        };
        Ok(smf)
    }

    /// Replaces the current document and adds the base layout
    fn setup_document(&mut self) -> Result<()> {
        let settings = self.svg_settings.clone().unwrap();
        self.svg = Document::new();
        self.svg = self.svg.clone().add(
            svg::node::element::Path::new()
                .set("stroke", settings.staff_bounding_box_colour.unwrap())
                .set("stroke-width", 3)
                .set(
                    "d",
                    svg::node::element::path::Data::new()
                        .move_to((settings.staff_offset, settings.staff_offset))
                        .line_by((20, 20))
                        .close(),
                ),
        );
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
        svg::save("image.svg", &self.svg).unwrap();
        Ok(())
    }
}
