// std
use std::{
    fs::{self, File},
    io::{BufReader, Read},
};

// midly
use midly::{MidiMessage, Smf, Track, TrackEvent, TrackEventKind};

// serde
use serde::{Serialize, Serializer};

// Internal
use super::MusicBoxConverter;
use crate::{
    music::{self, meta_information::MetaInformation, music_box::MusicBox, note::Note},
    prelude::*,
    settings::svg::SvgSettings,
    svg::{circle::Circle, document::Document, line::Line},
    vec2::Vec2,
};

impl MusicBoxConverter {
    pub fn run(mut self) -> Result<()> {
        self.choose_music_box()?;
        self.load_settings()?;
        // We can't make smf and bytes part of the MusicBoxConverter Struct because smf references bytes, so they need to have the same lifetime. ARGH!
        let bytes = self.get_bytes()?;
        let smf = Self::get_smf(&bytes)?;
        self.set_scale_factor_and_meta(&smf)?;
        self.generate_svgs(&smf);
        self.output_document()?;

        Ok(())
    }

    /// Deserializes ./box.json and assigns the MusicBox with the name given via arguments to the self.music_box.
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

    /// Deserializes ./svg_settings.json and assigns the deserialized SvgSettings to self.svg_settings.
    fn load_settings(&mut self) -> Result<()> {
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

    /// Returns a Vec<u8> with the data of the file.
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

    /// Returns a Smf which links to the bytes which were passed.
    fn get_smf(bytes: &[u8]) -> Result<Smf> {
        let smf: Smf = match Smf::parse(bytes) {
            Ok(t) => t,
            Err(e) => return Err(Error::MidiError(Box::new(e))),
        };

        Ok(smf)
    }

    /// Sets the scale_factor and meta in the self.
    fn set_scale_factor_and_meta(&mut self, smf: &Smf) -> Result<()> {
        // ----- Get Meta -----
        self.meta = Option::Some(MetaInformation::gather_meta(&smf.tracks[0]));
        let meta = self.meta.as_ref().unwrap();

        // ----- Get ScaleFactor -----
        let music_box = self.music_box.as_ref().unwrap();

        // X: How much do we have to scale the notes for it to be compatible with the music box
        // Y: How much space there is between two lines
        let mut scale_factor = Vec2::<f64>::new(
            music_box.min_note_distance_mm / <u32 as Into<f64>>::into(meta.min_distance_same_notes),
            music_box.vertical_note_distance(),
        );

        self.scale = Option::Some(scale_factor);

        Ok(())
    }

    /// Generates the svgs and saves them in self.svg.
    fn generate_svgs(&mut self, smf: &Smf) -> Result<()> {
        // References
        let meta = self.meta.as_ref().unwrap();
        let scale_factor = self.scale.clone().unwrap();
        let settings = self.svg_settings.as_ref().unwrap();

        // Output
        let mut pages = Vec::<Vec<TrackEvent>>::new();
        pages.push(Vec::<TrackEvent>::new());

        // Loop variables
        let mut length = 0f64;

        // Take track 0 for now. Might change later
        for event in smf.tracks[0].clone() {
            if f64::from(u32::from(event.delta)) * scale_factor.x + length > PAPER_SIZE.x {
                // Need to call this from here since we can't return data
                self.draw_page(&pages.last().unwrap(), length);

                pages.push(Vec::<TrackEvent>::new());
                length = 0f64;
            }

            // Only add the event if it is of kind Midi
            match event.kind {
                // Add
                TrackEventKind::Midi { channel, message } => {
                    if self
                        .music_box
                        .as_ref()
                        .unwrap()
                        .is_valid_note(&Note::from_midi_message(&message))
                    {
                        pages.last_mut().unwrap().push(event.clone());
                    }
                }
                // Ignore
                _ => (),
            };

            length += f64::from(u32::from(event.delta)) * scale_factor.x;
        }

        Ok(())
    }

    /// Don't call manually.
    /// It's called by <code>self.generate_svgs</code>.
    fn draw_page(&mut self, notes: &Vec<TrackEvent<'_>>, length: f64) -> Result<()> {
        // References
        let settings = self.svg_settings.as_ref().unwrap();
        let scale_factor = self.scale.as_ref().unwrap();
        let music_box = self.music_box.as_ref().unwrap();

        // Output
        let mut document = Document::default();

        // Draw note lines
        for i in 0..self.music_box.as_ref().unwrap().note_count() {
            let current_pos = settings.staff_offset_mm + (i as f64 * scale_factor.y);
            document.append(
                Line::new_builder()
                    .set_start(settings.staff_offset_mm, current_pos)
                    .set_end(length, current_pos)
                    .set_stroke(settings.staff_line_colour.clone())
                    .set_stroke_width(settings.staff_line_thickness_mm)
                    .finish(),
            );
        }

        // Draw notes
        let mut absolute_offset = 0f64;

        for note in notes {
            let current_delta = scale_factor.x * f64::from(u32::from(note.delta));
            let message = match note.kind {
                TrackEventKind::Midi { channel, message } => message,
                _ => panic!("Invalid state in draw_page()!"),
            };

            let note_std = &Note::from_midi_message(&message).unwrap();
            let note_line = music_box.get_index(note_std);

            if note_line.is_none() {
                if self.verbose {
                    println!("Can't play note: {:?}", note_std);
                }
            } else {
                document.append(
                    Circle::new_builder()
                        .set_centre(
                            current_delta + absolute_offset,
                            scale_factor.y * note_line.unwrap() as f64 + settings.staff_offset_mm,
                        )
                        .set_radius(settings.hole_radius_mm)
                        .set_stroke(settings.hole_colour.clone())
                        .set_stroke_width(settings.hole_radius_mm)
                        .finish(),
                );
            }

            absolute_offset += scale_factor.x * f64::from(u32::from(note.delta));
        }

        self.svg.push(document);
        Ok(())
    }

    /* /// Replaces the current document and adds the base layout
    fn draw_document(&mut self, smf: &Smf) -> Result<()> {
        // For reference
        let meta = self.meta.as_ref().unwrap();
        let scale_factor = self.scale.clone().unwrap();

        // Output
        let mut notes = Vec::<Vec<TrackEvent>>::new();
        let settings = self.svg_settings.as_ref().unwrap();
        notes.push(Vec::<TrackEvent>::new());

        // For the loop
        let length_const: f64 = 2f64 * settings.staff_offset_mm;
        let mut length: f64 = length_const;

        for event in &smf.tracks[0] {
            if f64::from(u32::from(event.delta)) * scale_factor.y + length > PAPER_SIZE.x {
                self.setup_page(notes.last().unwrap())?;

                notes.push(Vec::<TrackEvent>::new());
                length = length_const;
            }

            notes.last_mut().unwrap().push(*event);

            //todo!("Check if the notes can even be played");
        }
        self.setup_page(notes.last().unwrap())?;

        for (i, page) in notes.iter().enumerate() {
            self.draw_notes(&notes[i], i)?;
        }

        Ok(())
    }

    /// Gets called by draw_document(). Do not call manually
    fn setup_page(&mut self, events: &Vec<TrackEvent>) -> Result<()> {
        self.svg.push(Document::default());
        let mut svg = self.svg.last_mut().unwrap();
        let music_box = self.music_box.as_ref().unwrap();
        let settings = self.svg_settings.as_ref().unwrap();
        let scale_factor = self.scale.as_ref().unwrap();
        let mut length = 0u64;
        for event in events {
            length += u32::from(event.delta) as u64;
        }

        // Note lines

        for note_index in 0..music_box.note_count() {
            let current_pos = settings.staff_offset_mm + (note_index as f64 * scale_factor.y);
            svg.append(
                Line::new_builder()
                    .set_start(settings.staff_offset_mm, current_pos)
                    .set_end(length as f64, current_pos)
                    .set_stroke(settings.staff_line_colour.clone())
                    .set_stroke_width(3f64)
                    .finish(),
            )
        }

        Ok(())
    }

    /// Fills the current document with the notes
    fn draw_notes(&mut self, notes: &[TrackEvent], i: usize) -> Result<()> {
        for note in notes {
            todo!("Finish this...");
            //self.svg[i].append(Circle::new_builder().set_centre(x, y))
        }
        Ok(())
    } */

    /// Outputs the current document to the specified output file
    fn output_document(&self) -> Result<()> {
        let mut path_string = self.args.get_one::<String>("io_out").unwrap().to_owned();
        path_string = path_string.replace(' ', "");
        let path_buf = std::path::PathBuf::from(path_string);
        let path_canonical = match std::fs::canonicalize(path_buf) {
            Ok(t) => t,
            Err(e) => return Err(Error::IOError(Box::new(e))),
        };

        for (i, svg) in self.svg.iter().enumerate() {
            let mut path_i = path_canonical.clone();
            path_i.push(i.to_string() + ".svg");
            svg.save(std::path::Path::new(&path_i))?
        }
        Ok(())
    }
}

pub const PAPER_SIZE: Vec2<f64> = Vec2 {
    x: 297f64,
    y: 210f64,
};
