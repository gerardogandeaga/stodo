use std::fmt;
use std::fmt::Formatter;
use std::fs;
use std::fs::{ReadDir};
use std::path::{PathBuf};
use regex::Regex;
use lazy_static::lazy_static;

/*
A Stodo represents a file with the stodo strings
 */
#[derive(Debug)]
pub struct StodoFile {
    path: PathBuf,          // source file path
    stodo_entries: Vec<StodoEntry>, // all the detected TODOs in the file
}

#[derive(Debug)]
pub struct StodoEntry(String, u32);

impl fmt::Display for StodoFile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "n: {}, p: {}", self.stodo_entries.len(), self.path.display().to_string())
    }
}

impl StodoFile {

    /*
    Returns a vector of stodo objects for the current directory
     */
    pub fn from_dir(dir_path: &PathBuf) -> Option<Vec<Self>> {
        let paths: ReadDir = fs::read_dir(dir_path).unwrap();

        let stodos: Vec<Self> = paths.filter_map(|entry| {
            let path: PathBuf = entry.unwrap().path();

            if path.is_file() {
                Self::from_file(path)
            }
            else {
                None
            }
        })
        .collect();

        if stodos.is_empty() {
            None
        }
        else {
            Some(stodos)
        }
    }

    /*
    Analyze and return a stodo struct if the input file has one
     */
    pub fn from_file(path: PathBuf) -> Option<Self> {
        lazy_static!{
            // TODO: optimize the regex? https://github.com/rust-lang/regex/blob/master/PERFORMANCE.md
            static ref STODO_REGEX: Regex = Regex::new(r"^([^a-zA-Z0-9]+|\s*)\b[Tt][Oo][Dd][Oo]\b.*").unwrap();
        }

        let read_result = fs::read_to_string(&path);
        if read_result.is_err() {
            // println!("Error reading -> {}", path.display());
            return None;
        }

        // /Users/gerardogandeaga/Dev/cli/stodo/target/debug/.fingerprint/serde_json-47ac3846cebe7691

        let contents = read_result.unwrap();
        let mut str_todos: Vec<StodoEntry> = vec![];

        // pattern match to find the todos in a file
        contents.split("\n").enumerate().for_each(|(i, line)| {
            // use Regex::find?
            if STODO_REGEX.is_match(line) {
                str_todos.push(StodoEntry(String::from(line.trim()), i as u32 + 1));
            }
        });

        if str_todos.is_empty() {
            None
        }
        else {
            Some(Self {
                path: PathBuf::from(&path),
                stodo_entries: str_todos
            })
        }
    }

    pub fn file_path(&self) -> &PathBuf {
        &self.path
    }

    pub fn stodo_entries(&self) -> &Vec<StodoEntry> {
        &self.stodo_entries
    }

}

impl StodoEntry {

    pub fn stodo_string(&self) -> String {
        String::from(&self.0)
    }

    pub fn line_number(&self) -> u32 {
        self.1
    }
}
