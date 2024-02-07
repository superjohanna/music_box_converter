// std
use std::{
    fs::{self, File},
    io::{BufReader, Read, Write},
};

// simplelog
use simplelog::*;

// midly
use midly::{MidiMessage, Smf, Track as MidiTrack, TrackEvent, TrackEventKind};

// serde
use serde::{Serialize, Serializer};

// Internal
use super::MusicBoxConvert;
use crate::{
    music::{
        self,
        event::Event,
        music_box::MusicBox,
        note::Note,
        track::{self, Track},
    },
    prelude::*,
    settings::Settings,
    svg_writer::{circle::Circle, document::Document, line::Line},
    vec2::Vec2,
};

impl MusicBoxConvert {
    pub fn run_output_file(mut self) -> Result<()> {
        self.initiate_logger()?;
        self.load_music_box()?;
        self.load_settings()?;
        self.get_abs()?;
        self.set_scale_factor()?;
        self.generate_svgs()?;
        self.write_documents()?;

        Ok(())
    }

    pub fn run_output_string(mut self) -> Result<Vec<String>> {
        self.initiate_logger()?;
        self.load_music_box()?;
        self.load_settings()?;
        self.get_abs()?;
        self.set_scale_factor()?;
        self.generate_svgs()?;
        self.output_documents()
    }

    /// Initiates the logger with the correct log level. The logger is static and so this musn't be called more than once
    fn initiate_logger(&mut self) -> Result<()> {
        let verbosity = self.args.get_count("verbosity");
        let quiet = self.args.get_flag("quiet");
        if quiet {
            TermLogger::init(
                LevelFilter::Off,
                Config::default(),
                TerminalMode::Mixed,
                ColorChoice::Auto,
            );
            Ok(())
        } else {
            TermLogger::init(
                match verbosity {
                    0 => LevelFilter::Warn,
                    1 => LevelFilter::Info,
                    2 => LevelFilter::Debug,
                    _ => LevelFilter::Trace,
                },
                Config::default(),
                TerminalMode::Mixed,
                ColorChoice::Always,
            );
            debug!(
                "Verbosity set to {}",
                match verbosity {
                    2 => "Debug",
                    _ => "Trace",
                }
            );
            Ok(())
        }
    }

    /// Deserializes ./box.json and assigns the MusicBox with the name given via arguments to the self.music_box.
    fn load_music_box(&mut self) -> Result<()> {
        let file = match File::open(self.args.get_one::<String>("io_box").unwrap()) {
            Ok(t) => t,
            Err(e) => {
                return Err(Error::IOError(
                    Box::new(e),
                    Box::new(self.args.get_one::<String>("io_box").unwrap().to_string()),
                ))
            }
        };

        let deserialized: MusicBox = match serde_json::from_reader(BufReader::new(file)) {
            Ok(t) => t,
            Err(e) => return Err(Error::SerdeJsonError(Box::new(e))),
        };

        self.music_box = Some(deserialized);
        Ok(())
    }

    /// Deserializes ./svg_settings.json and assigns the deserialized SvgSettings to self.svg_settings.
    fn load_settings(&mut self) -> Result<()> {
        let file = match File::open(self.args.get_one::<String>("io_settings").unwrap()) {
            Ok(t) => t,
            Err(e) => {
                return Err(Error::IOError(
                    Box::new(e),
                    Box::new(self.args.get_one::<String>("io_settings").unwrap().clone()),
                ))
            }
        };

        let deserialized: Settings = match serde_json::from_reader(BufReader::new(file)) {
            Ok(t) => t,
            Err(e) => return Err(Error::SerdeJsonError(Box::new(e))),
        };

        self.settings = Some(deserialized);
        Ok(())
    }

    /// Stores an absolute representation of the midi data in self.track. Also output a midi file including all track plus an extra one containing the possibly transposed track.
    fn get_abs(&mut self) -> Result<()> {
        let mut input = self.args.get_one::<String>("io_in").unwrap().to_owned();
        let track_number = self.args.get_one::<usize>("track").unwrap().to_owned();
        let transpose = self.args.get_flag("transpose");

        if input.chars().collect::<Vec<char>>()[0] == ' ' {
            input.remove(0);
        }

        let mut file: Vec<u8> = match std::fs::read(input.clone()) {
            Ok(t) => t,
            Err(e) => return Err(Error::IOError(Box::new(e), Box::new(input))),
        };

        let smf: Smf = match Smf::parse(&file) {
            Ok(t) => t,
            Err(e) => return Err(Error::MidiError(Box::new(e))),
        };

        if track_number > smf.tracks.len() - 1 {
            return Err(Error::Generic(format!(
                "File only contains {0} track(s). Track number {1} is out of bounds. Remember that the track number is zero-based: 0 => track number 1, 3 => track number 4",
                smf.tracks.len(),
                track_number
            )));
        }

        self.track = Some(Track::from_midi_track(
            smf.tracks[track_number].clone(),
            self.music_box.res()?,
            &transpose,
        ));

        if self.track.res()?.len() < 2 {
            return Err(Error::Generic(format!(
                "Track {} contains fewer than two playable notes",
                track_number
            )));
        }

        if let Some(t) = self.args.get_one::<String>("io_out_midi") {
            let mut abs_path = match crate::path::absolute_path(t) {
                Ok(t) => t,
                Err(e) => return Err(Error::IOError(Box::new(e), Box::new(t.clone()))),
            };

            let parent = abs_path.parent();

            if let Some(t) = parent {
                match std::fs::create_dir_all(t) {
                    Ok(t) => t,
                    Err(e) => (), /*return Err(Error::IOError(Box::new(e)))*/
                }
            }

            let mut file = match File::create(abs_path) {
                Ok(t) => t,
                Err(e) => return Err(Error::IOError(Box::new(e), Box::new(t.clone()))),
            };

            let track = self
                .track
                .clone()
                .unwrap()
                .to_midi_track(smf.tracks[0].clone());
            let mut midi = smf.clone();
            let mut buf = Vec::<u8>::new();
            midi.tracks.push(track);
            midi.write(&mut buf);
            file.write_all(&buf);
        }

        Ok(())
    }

    /// Sets the scale_factor and meta in the self.
    fn set_scale_factor(&mut self) -> Result<()> {
        let music_box = self.music_box.res()?;

        // X (Horizontal): How much do we have to scale the notes for it to be compatible with the music box
        // Y (Vertical): How much space there is between two lines
        let mut scale_factor = Vec2::<f64>::new(
            music_box.min_note_distance_mm / self.track.res()?.min_distance() as f64,
            music_box.vertical_note_distance(),
        );

        self.scale = Option::Some(scale_factor);

        Ok(())
    }

    /// Generates the svgs and saves them in self.svg.
    fn generate_svgs(&mut self) -> Result<()> {
        // Pages
        let mut pages = Vec::<Vec<Event>>::new();
        pages.push(Vec::<Event>::new());

        // Loop variables
        let mut first_note_abs = u64::MIN;
        let mut overflow_notes = u64::MIN;
        let mut overflow_sprockets = 0f64;

        for event in self.track.clone().unwrap().iter() {
            if event.vel == 0 {
                continue;
            }
            if (event.abs - first_note_abs + overflow_notes) as f64 * self.scale.res()?.x
                + self.settings.res()?.staff_offset_mm
                > self.settings.res()?.paper_size_x
            {
                overflow_sprockets =
                    self.draw_page(pages.last().unwrap(), overflow_notes, overflow_sprockets)?;
                overflow_notes = event.abs - pages.last().unwrap().last().unwrap().abs;
                pages.push(Vec::<Event>::new());
                first_note_abs = event.abs;
            }

            pages.last_mut().unwrap().push(event.clone());
        }

        self.draw_page(pages.last().unwrap(), overflow_notes, overflow_sprockets)?;

        Ok(())
    }

    /// Don't call manually.
    /// It's called by <code>self.generate_svgs</code>.
    fn draw_page(
        &mut self,
        notes: &Vec<Event>,
        overflow_notes: u64,
        overflow_sprockets: f64,
    ) -> Result<f64> {
        // Output
        let mut document = Document::default();

        // Draw note lines
        for i in 0..self.music_box.res()?.note_count() {
            let current_pos =
                self.settings.res()?.staff_offset_mm + (i as f64 * self.scale.res()?.y);
            document.append(
                Line::new_builder()
                    .set_start(self.settings.res()?.staff_offset_mm, current_pos)
                    .set_end(
                        (notes.last().unwrap().abs - notes.first().unwrap().abs + overflow_notes)
                            as f64
                            * self.scale.res()?.x
                            + self.settings.res()?.staff_offset_mm,
                        current_pos,
                    )
                    .set_stroke(self.settings.res()?.staff_line_colour.clone())
                    .set_stroke_width(self.settings.res()?.staff_line_thickness_mm)
                    .finish(),
            );
        }

        // Draw staff bounding box
        // Left
        document.append(
            Line::new_builder()
                .set_start(
                    self.settings.res()?.staff_offset_mm,
                    self.settings.res()?.staff_offset_mm
                        - self
                            .settings
                            .res()?
                            .staff_bounding_box_top_bottom_distance_mm,
                )
                .set_end(
                    self.settings.res()?.staff_offset_mm,
                    self.scale.res()?.y * (self.music_box.res()?.note_count() as f64 - 1f64)
                        + self.settings.res()?.staff_offset_mm
                        + self
                            .settings
                            .res()?
                            .staff_bounding_box_top_bottom_distance_mm,
                )
                .set_stroke(
                    self.settings
                        .res()?
                        .staff_bounding_box_left_right_colour
                        .clone(),
                )
                .set_stroke_width(self.settings.res()?.staff_bounding_box_thickness_mm)
                .finish(),
        );

        // Right
        document.append(
            Line::new_builder()
                .set_start(
                    (notes.last().unwrap().abs - notes.first().unwrap().abs + overflow_notes)
                        as f64
                        * self.scale.res()?.x
                        + self.settings.res()?.staff_offset_mm,
                    self.settings.res()?.staff_offset_mm
                        - self
                            .settings
                            .res()?
                            .staff_bounding_box_top_bottom_distance_mm,
                )
                .set_end(
                    (notes.last().unwrap().abs - notes.first().unwrap().abs + overflow_notes)
                        as f64
                        * self.scale.res()?.x
                        + self.settings.res()?.staff_offset_mm,
                    self.scale.res()?.y * (self.music_box.res()?.note_count() as f64 - 1f64)
                        + self.settings.res()?.staff_offset_mm
                        + self
                            .settings
                            .res()?
                            .staff_bounding_box_top_bottom_distance_mm,
                )
                .set_stroke(
                    self.settings
                        .res()?
                        .staff_bounding_box_left_right_colour
                        .clone(),
                )
                .set_stroke_width(self.settings.res()?.staff_bounding_box_thickness_mm)
                .finish(),
        );

        // Top
        document.append(
            Line::new_builder()
                .set_start(
                    self.settings.res()?.staff_offset_mm,
                    self.settings.res()?.staff_offset_mm
                        - self
                            .settings
                            .res()?
                            .staff_bounding_box_top_bottom_distance_mm,
                )
                .set_end(
                    (notes.last().unwrap().abs - notes.first().unwrap().abs + overflow_notes)
                        as f64
                        * self.scale.res()?.x
                        + self.settings.res()?.staff_offset_mm,
                    self.settings.res()?.staff_offset_mm
                        - self
                            .settings
                            .res()?
                            .staff_bounding_box_top_bottom_distance_mm,
                )
                .set_stroke(
                    self.settings
                        .res()?
                        .staff_bounding_box_top_bottom_colour
                        .clone(),
                )
                .set_stroke_width(self.settings.res()?.staff_bounding_box_thickness_mm)
                .finish(),
        );

        // Bottom
        document.append(
            Line::new_builder()
                .set_start(
                    self.settings.res()?.staff_offset_mm,
                    self.scale.res()?.y * (self.music_box.res()?.note_count() as f64 - 1f64)
                        + self.settings.res()?.staff_offset_mm
                        + self
                            .settings
                            .res()?
                            .staff_bounding_box_top_bottom_distance_mm,
                )
                .set_end(
                    (notes.last().unwrap().abs - notes.first().unwrap().abs + overflow_notes)
                        as f64
                        * self.scale.res()?.x
                        + self.settings.res()?.staff_offset_mm,
                    self.scale.res()?.y * (self.music_box.res()?.note_count() as f64 - 1f64)
                        + self.settings.res()?.staff_offset_mm
                        + self
                            .settings
                            .res()?
                            .staff_bounding_box_top_bottom_distance_mm,
                )
                .set_stroke(
                    self.settings
                        .res()?
                        .staff_bounding_box_top_bottom_colour
                        .clone(),
                )
                .set_stroke_width(self.settings.res()?.staff_bounding_box_thickness_mm)
                .finish(),
        );

        // Draw notes
        let first_note_pos = notes.first().unwrap().abs;
        let mut stop_point_sprocket = u64::MIN;

        for event in notes {
            info!("Drawing {}", event.note);

            let note_index = match self.music_box.res()?.get_index(&event.note) {
                Some(t) => t + 1, // Zero based index
                None => continue,
            };

            document.append(
                Circle::new_builder()
                    .set_centre(
                        (event.abs + overflow_notes - first_note_pos) as f64 * self.scale.res()?.x
                            + self.settings.res()?.staff_offset_mm,
                        (self.music_box.res()?.note_count() - note_index) as f64
                            * self.scale.res()?.y
                            + self.settings.res()?.staff_offset_mm,
                    )
                    .set_radius(self.settings.res()?.hole_radius_mm)
                    .set_fill(self.settings.res()?.hole_colour.clone())
                    .finish(),
            );

            stop_point_sprocket = event.abs - first_note_pos + overflow_notes;
        }

        let mut current_x = 0f64;

        info!("Drawing sprocket holes");
        // Draw sprocket holes
        if self.settings.res()?.sprocket_hole_enable {
            // Y position of the top holes
            let top_y = self.settings.res()?.staff_offset_mm
                - self.settings.res()?.sprocket_hole_distance_staff_mm;

            // Y position of the bottom holes
            let bot_y = self.settings.res()?.staff_offset_mm
                + ((self.music_box.res()?.note_count() - 1) as f64 * self.scale.res()?.y)
                + self.settings.res()?.sprocket_hole_distance_staff_mm;

            let page_size_x = self.settings.res()?.paper_size_x;
            let sprocket_area =
                self.scale.res()?.x * stop_point_sprocket as f64 - overflow_sprockets;
            for i in
                0..=(sprocket_area / self.settings.res()?.sprocket_hole_distance_mm).floor() as u64
            {
                current_x = (self.settings.res()?.staff_offset_mm + overflow_sprockets)
                    + (i as f64 * self.settings.res()?.sprocket_hole_distance_mm);

                // Top hole
                document.append(
                    Circle::new_builder()
                        .set_centre(current_x, top_y)
                        .set_radius(self.settings.res()?.hole_radius_mm)
                        .set_fill(self.settings.res()?.hole_colour.clone())
                        .finish(),
                );

                // Bottom hole
                document.append(
                    Circle::new_builder()
                        .set_centre(current_x, bot_y)
                        .set_radius(self.settings.res()?.hole_radius_mm)
                        .set_fill(self.settings.res()?.hole_colour.clone())
                        .finish(),
                );
            }
        }

        self.svg.push(document);

        // Return the overflow of the sprocket holes
        Ok(self.settings.res()?.paper_size_x - self.settings.res()?.staff_offset_mm - current_x)
    }

    /// Writes the documents to a file
    fn write_documents(&self) -> Result<()> {
        let mut path_string = self.args.get_one::<String>("io_out").unwrap().to_owned();
        let mut abs_path = match crate::path::absolute_path(path_string.clone()) {
            Ok(t) => t,
            Err(e) => return Err(Error::IOError(Box::new(e), Box::new(path_string))),
        };

        match std::fs::create_dir_all(abs_path.clone()) {
            Ok(t) => t,
            Err(e) => return Err(Error::IOError(Box::new(e), Box::new(path_string))),
        }

        for (i, svg) in self.svg.iter().enumerate() {
            let mut path_i = abs_path.clone();
            path_i.push(i.to_string() + ".svg");
            svg.save(std::path::Path::new(&path_i))?
        }

        Ok(())
    }

    /// Returns a Vec of strings containing the documents
    fn output_documents(&self) -> Result<Vec<String>> {
        let mut docs = Vec::<String>::new();

        for svg in self.svg.iter() {
            docs.push(svg.print());
        }

        Ok(docs)
    }
}
