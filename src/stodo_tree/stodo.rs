use std::fmt;
use std::fmt::Formatter;
use std::fs;
use std::fs::{DirEntry, ReadDir};
use std::path::{PathBuf};
use regex::Regex;

/*
A Stodo represents a file with the stodo strings
 */
#[derive(Debug)]
pub struct StodoFile {
    path: PathBuf,          // source file path
    stodo_entries: Vec<StodoEntry>, // all the detected TODOs in the file
}

#[derive(Debug)]
pub struct StodoEntry {
    stodo: String,
    line: u32,
}

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
        let mut todos: Vec<Self> = vec![];

        for path in paths {
            let entry: DirEntry = path.unwrap();

            // skip if the entry is not a file
            if !entry.path().is_file() {
                continue
            }

            let todo: Option<Self> = Self::from_file(entry.path());

            if todo.is_some() {
                todos.push(todo.unwrap());
            }
        }

        if todos.is_empty() {
            None
        }
        else {
            Some(todos)
        }
    }

    /*
    Analyze and return a stodo struct if the input file has one
     */
    pub fn from_file(path: PathBuf) -> Option<Self> {
        let read_result = fs::read_to_string(&path);
        if read_result.is_err() {
            return None;
        }

        let contents = read_result.unwrap();
        let mut str_todos: Vec<StodoEntry> = vec![];

        // pattern match to find the todos in a file
        let re = Regex::new(r"^([^a-zA-Z0-9]+|\s*)\b[Tt][Oo][Dd][Oo]\b.*").unwrap();
        let mut line_i: u32 = 1;
        for line in contents.split("\n") {
            if re.is_match(line) {
                str_todos.push(StodoEntry {
                    stodo: String::from(line.trim()),
                    line: line_i,
                });
            }
            line_i += 1;
        }

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
        String::from(&self.stodo)
    }

    pub fn line_number(&self) -> u32 {
        self.line
    }
}
