use std::fmt;
use std::fmt::Formatter;
use std::fs;
use std::fs::{DirEntry, ReadDir};
use std::path::{Path, PathBuf};
use regex::Regex;

/*
A Stodo represents a file with the stodo strings
 */
#[derive(Debug)]
pub struct StodoFile {
    pub path: PathBuf,          // source file path
    pub str_todos: Vec<String>, // all the detected TODOs in the file
}

impl fmt::Display for StodoFile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "n: {}, p: {}", self.str_todos.len(), self.path.display().to_string())
    }
}

impl StodoFile {

    /*
    Returns a vector of stodo objects for the current directory
     */
    pub fn from_dir(dir_path: &PathBuf) -> Option<Vec<StodoFile>> {
        let paths: ReadDir = fs::read_dir(dir_path).unwrap();
        let mut todos: Vec<StodoFile> = vec![];

        for path in paths {
            let entry: DirEntry = path.unwrap();

            // skip if the entry is not a file
            if !entry.path().is_file() {
                continue
            }

            let todo: Option<StodoFile> = StodoFile::from_file(entry);

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
    fn from_file(entry: DirEntry) -> Option<StodoFile> {
        let read_result = fs::read_to_string(entry.path());
        if read_result.is_err() {
            return None;
        }

        let contents = read_result.unwrap();
        let mut str_todos: Vec<String> = vec![];

        // pattern match to find the todos in a file
        let re = Regex::new(r"^([^a-zA-Z0-9]+|\s*)\b[Tt][Oo][Dd][Oo]\b.*").unwrap();
        let mut line_i: u32 = 1;
        for line in contents.split("\n") {
            if re.is_match(line) {
                let mut todo_line = line_i.to_string() + ": ";
                todo_line.push_str(line.trim());
                str_todos.push(todo_line);
            }
            line_i += 1;
        }

        if str_todos.is_empty() {
            None
        }
        else {
            Some(StodoFile {
                path: PathBuf::from(&entry.path()),
                str_todos: str_todos
            })
        }
    }
}
