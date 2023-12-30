// std
use std::{
    fs::{self, File},
    io::{BufReader, Read},
};

// midly
use midly::{MidiMessage, Smf, Track as MidiTrack, TrackEvent, TrackEventKind};

// serde
use serde::{Serialize, Serializer};

// Internal
use super::MusicBoxConverter;
use crate::{
    music::{self, event::Event, music_box::MusicBox, note::Note, track::Track},
    prelude::*,
    settings::settings::Settings,
    svg::{circle::Circle, document::Document, line::Line},
    vec2::Vec2,
};

impl MusicBoxConverter {
    pub fn run(mut self) -> Result<()> {
        self.choose_music_box()?;
        self.load_settings()?;
        self.get_abs()?;
        self.set_scale_factor()?;
        self.generate_svgs()?;
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
        let file = match File::open(self.args.get_one::<String>("io_settings").unwrap()) {
            Ok(t) => t,
            Err(e) => return Err(Error::IOError(Box::new(e))),
        };

        let deserialized: Settings = match serde_json::from_reader(BufReader::new(file)) {
            Ok(t) => t,
            Err(e) => return Err(Error::SerdeJsonError(Box::new(e))),
        };

        self.svg_settings = Some(deserialized);
        Ok(())
    }

    /// Stores an absolute representation of the midi data in self.absolute_track
    fn get_abs(&mut self) -> Result<()> {
        let mut arg = self.args.get_one::<String>("io_in").unwrap().to_owned();
        if arg.chars().collect::<Vec<char>>()[0] == ' ' {
            arg.remove(0);
        }

        let mut file: Vec<u8> = match std::fs::read(arg) {
            Ok(t) => t,
            Err(e) => return Err(Error::IOError(Box::new(e))),
        };

        let smf: Smf = match Smf::parse(&file) {
            Ok(t) => t,
            Err(e) => return Err(Error::MidiError(Box::new(e))),
        };

        // Just taking the first track for now
        self.abs_track = Option::Some(Track::from_midi_track(smf.tracks[0].clone()));

        Ok(())
    }

    /// Sets the scale_factor and meta in the self.
    fn set_scale_factor(&mut self) -> Result<()> {
        let music_box = self.music_box.as_ref().unwrap();

        // X (Horizontal): How much do we have to scale the notes for it to be compatible with the music box
        // Y (Vertical): How much space there is between two lines
        let mut scale_factor = Vec2::<f64>::new(
            music_box.min_note_distance_mm / self.abs_track.as_ref().unwrap().min_distance() as f64,
            music_box.vertical_note_distance(),
        );

        self.scale = Option::Some(scale_factor);

        Ok(())
    }

    /// Generates the svgs and saves them in self.svg.
    fn generate_svgs(&mut self) -> Result<()> {
        // References
        let abs = self.abs_track.as_ref().unwrap();
        let scale_factor = self.scale.clone().unwrap();
        let settings = self.svg_settings.as_ref().unwrap();

        // Pages
        let mut pages = Vec::<Vec<Event>>::new();
        pages.push(Vec::<Event>::new());

        // Loop variables
        // let mut length = 0f64; // Length of the current notes on page
        let mut first_note_pos = u64::MIN; // Absolute position of the first note on the page
        let mut last_note_pos = u64::MIN;

        // Loop over every note and try to fit it onto a page. If the page is full add a new one
        for event in &**abs.clone() {
            if (event.abs - first_note_pos) as f64 * scale_factor.x > PAPER_SIZE.x {
                self.draw_page(
                    &pages.last().unwrap(),
                    (event.abs - first_note_pos) as f64 * scale_factor.x,
                );
                pages.push(Vec::<Event>::new());
                first_note_pos = event.abs;
            }

            pages.last_mut().unwrap().push(event.clone());
            last_note_pos = event.abs;
        }
        self.draw_page(
            &pages.last().unwrap(),
            (last_note_pos - first_note_pos) as f64 * scale_factor.x,
        );

        Ok(())
    }

    /// Don't call manually.
    /// It's called by <code>self.generate_svgs</code>.
    fn draw_page(&mut self, notes: &Vec<Event>, length: f64) -> Result<()> {
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
        let first_note_pos = notes.first().unwrap().abs;

        for event in notes {
            if self.verbose {
                println!("Playing {:?}", event.note);
            }
            let note_index = match music_box.get_index(&event.note) {
                Some(t) => t,
                None => continue,
            };

            document.append(
                Circle::new_builder()
                    .set_centre(
                        (event.abs - first_note_pos) as f64 * scale_factor.x
                            + settings.staff_offset_mm,
                        (music_box.note_count() - note_index) as f64 * scale_factor.y
                            + settings.staff_offset_mm,
                    )
                    .set_radius(settings.hole_radius_mm)
                    .set_stroke(settings.hole_colour.clone())
                    .set_stroke_width(settings.hole_radius_mm)
                    .finish(),
            );
        }

        self.svg.push(document);

        Ok(())
    }

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
