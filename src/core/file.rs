use std::fs;
use std::path::{PathBuf};
use regex::Regex;
use lazy_static::lazy_static;
use super::entry::StodoEntry;

/*
A Stodo represents a file with the stodo strings
 */
#[derive(Debug)]
pub struct StodoFile {
    path: PathBuf,                  // source file path
    stodo_entries: Vec<StodoEntry>, // all the detected TODOs in the file
}

impl StodoFile {

    /// Analyze and return a stodo struct if the input file has one
    pub fn from_file(path: PathBuf) -> Option<Self> {
        lazy_static!{
            // TODO: optimize the regex? https://github.com/rust-lang/regex/blob/master/PERFORMANCE.md
            static ref STODO_REGEX: Regex = Regex::new(r"^([^a-zA-Z0-9]+|\s*)\b[Tt][Oo][Dd][Oo]\b.*").unwrap();
            static ref FIXME_REGEX: Regex = Regex::new(r"^([^a-zA-Z0-9]+|\s*)\b[Ff][Ii][Xx][Mm][Ee]\b.*").unwrap();
        }

        let read_result = fs::read_to_string(&path);
        if read_result.is_err() {
            return None;
        }

        let contents = read_result.unwrap();
        let mut str_todos: Vec<StodoEntry> = vec![];

        // pattern match to find the todos in a file
        contents.split("\n").enumerate().for_each(|(i, line)| {
            // use Regex::find?
            if STODO_REGEX.is_match(line) {
                str_todos.push(StodoEntry::todo(String::from(line.trim()), i as u32 + 1));
            }
            else
            if FIXME_REGEX.is_match(line) {
                str_todos.push(StodoEntry::fixme(String::from(line.trim()), i as u32 + 1));
            }
        });

        Some(Self {
            path: PathBuf::from(&path),
            stodo_entries: str_todos
        })
    }
}

impl StodoFile {
    pub fn file_path(&self) -> &PathBuf {
        &self.path
    }

    pub fn stodo_entries(&self) -> &Vec<StodoEntry> {
        &self.stodo_entries
    }

    pub fn is_empty(&self) -> bool {
        self.stodo_entries.is_empty()
    }
}
