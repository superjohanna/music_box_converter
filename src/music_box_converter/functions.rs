// std
use std::{fs::File, io::BufReader};

// serde
use serde::{Serialize, Serializer};

// Internal
use super::MusicBoxConverter;
use crate::{music::music_box::MusicBox, prelude::*};

impl MusicBoxConverter {
    pub fn run(mut self) -> Result<()> {
        self.choose_music_box()?;
        Ok(())
    }

    /// Deserializes ./box.json and assigns the MusicBox with the name given via arguments to the self.music_box
    fn choose_music_box(mut self) -> Result<()> {
        let file = match File::open("./box.json") {
            Ok(t) => t,
            Err(e) => return Err(Error::IOError(Box::new(e))),
        };

        let deserialized: Vec<MusicBox> = match serde_json::from_reader(BufReader::new(file)) {
            Ok(t) => t,
            Err(e) => return Err(Error::SerdeJsonError(Box::new(e))),
        };

        let chosen_box: &String = match self.args.get_one("io_box") {
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
            self.music_box = Some(m);
            return Ok(());
        }

        Err(Error::Generic(
            format!("Music box '{0}' not found!", *chosen_box).to_string(),
        ))
    }
}
