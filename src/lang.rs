use std::{collections::HashMap, fs::File, io::BufReader};

use serde_derive::{Deserialize, Serialize};
use serde_with::serde_as;
use simplelog::warn;

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct LangMap(#[serde_as(as = "Vec<(_, _)>")] HashMap<String, String>);

impl std::ops::Deref for LangMap {
    type Target = HashMap<String, String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for LangMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl LangMap {
    pub fn load_from_fs(path: &str) -> Self {
        let file = match File::open(path) {
            Ok(t) => t,
            Err(_) => return fallback_lang(),
        };

        let buf = BufReader::new(file);

        match serde_json::from_reader(buf) {
            Ok(t) => t,
            Err(_) => fallback_lang(),
        }
    }

    // Returns the cloned value at key 'key' or 'key' if there is none
    pub fn val_at(&self, key: &str) -> String {
        match self.get_key_value(key) {
            Some(t) => t.1.clone(),
            None => key.to_owned(),
        }
    }
}

fn fallback_lang() -> LangMap {
    warn!("Couldn't find language file for your locale. Using fallback language");

    #[cfg(not(target_os = "windows"))]
    let ret = serde_json::from_str(include_str!("../lang/en-GB.json")).unwrap();

    #[cfg(target_os = "windows")]
    let ret = serde_json::from_str(include_str!("..\\lang\\en-GB.json")).unwrap; // Don't know if this works. Don't have a windows computer

    ret
}

#[cfg(test)]
mod tests {
    use super::LangMap;

    #[test]
    fn lang() {
        let lang = LangMap::load_from_fs("fallback"); // Not a real path, so it uses the fallback hashmap

        assert_eq!(lang.get_key_value("MusicBox").unwrap().1, "music box");
    }

    #[test]
    fn serialize() {
        let mut hash = LangMap::default();
        hash.insert("k".to_string(), "v".to_string());
        let s = serde_json::to_string_pretty(&hash).unwrap();
        println!("{}", s);
    }
}
