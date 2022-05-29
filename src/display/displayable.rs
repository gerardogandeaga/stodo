/// implements displayable traits for the Stodo core structs
// use std::fmt;
// use std::fmt::{Formatter};
use std::path::PathBuf;
use ansi_term::Colour;
use crate::core::{StodoDir, StodoFile, StodoEntry};

pub trait Displayable {
    fn to_displayable(&self) -> String;
}

impl Displayable for StodoDir {
    fn to_displayable(&self) -> String {
        format!("{}", Colour::Cyan
            .paint(basename(self.in_path()))
            .to_string())
    }
}

impl Displayable for StodoFile {
    fn to_displayable(&self) -> String {

        if !self.is_empty() {
            format!("{}", Colour::White
                .paint(basename(self.file_path()))
                .to_string())
        }
        else {
            format!("{}", Colour::Red
                .paint(
                    format!("{}: nothing found.", basename(self.file_path()))
                )
                .to_string())
        }
    }
}

impl Displayable for StodoEntry {
    fn to_displayable(&self) -> String {
        format!("{}", Colour::RGB(34, 140, 34)
            .italic()
            .paint(self.stodo_string())
            .to_string())
    }
}

fn basename(path: &PathBuf) -> String {

    let mut basename = String::from(path.file_name().unwrap_or_default().to_str().unwrap());

    if path.is_dir() {
        basename.push('/');
    }

    basename
}
