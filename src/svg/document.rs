use crate::prelude::*;
use std::{fmt::Debug, fs::File, io::Write};

pub trait Child {
    fn clone_dyn(&self) -> Box<dyn Child>;
    fn fmt_dyn(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    fn print(&self, unit_suffix: String) -> String;
}

#[derive(Clone, Debug)]
pub struct Document {
    children: Vec<Box<dyn Child>>,
    unix_suffix: String,
}

impl Clone for Box<dyn Child> {
    fn clone(&self) -> Self {
        self.clone_dyn()
    }
}

impl Debug for Box<dyn Child> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_dyn(f)
    }
}

impl Default for Document {
    /// Default unit is millimetres
    fn default() -> Self {
        Self {
            children: Default::default(),
            unix_suffix: "mm".to_string(),
        }
    }
}

impl Document {
    pub fn append(&mut self, child: Box<dyn Child>) -> &mut Self {
        self.children.push(child);
        self
    }

    pub fn print(&self) -> String {
        let start = r#"<svg version="1.1" xmlns="http://www.w3.org/2000/svg">"#.to_string();
        let end = r#"</svg>"#;
        let mut content = String::default();

        for element in &self.children {
            content += &element.print(self.unix_suffix.clone());
            content += "\n"
        }

        format!("{start}\n{content}{end}")
    }

    pub fn save(&self, path: &std::path::Path) -> Result<()> {
        let mut file = match File::create(path) {
            Ok(t) => t,
            Err(e) => return Err(Error::IOError(Box::new(e))),
        };

        match file.write_all(self.print().as_bytes()) {
            Ok(t) => (),
            Err(e) => return Err(Error::IOError(Box::new(e))),
        }

        Ok(())
    }
}
