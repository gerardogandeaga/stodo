use std::path::PathBuf;
use super::StodoFile;
use std::hash::{Hash};

/*
Represents a directory with files that have valid todo strings in them
TODO: only keep the relative path of the directories below
 */
#[derive(Debug)]
pub struct StodoDir {
    path: PathBuf,          // path that the user entered, could be relative, could be absolute
    stodos: Vec<StodoFile>, // todos in this directory
    is_empty: bool,         // If the stodo dir does not have any stodos in the current or sub directories
    is_root: bool
}

#[allow(dead_code)]
impl StodoDir {

    pub fn new(path: PathBuf, is_root: bool) -> Self {
        Self {
            path,
            stodos: vec![],
            is_empty: true,
            is_root
        }
    }
}

impl StodoDir {

    pub fn stodos(&self) -> &Vec<StodoFile> {
        &self.stodos
    }

    pub fn in_path(&self) -> &PathBuf {
        &self.path
    }

    pub fn set_not_empty(&mut self) {
        self.is_empty = false;
    }

    pub fn empty_stodos(&self) -> bool {
        self.stodos.is_empty()
    }

    pub fn empty(&self) -> bool {
        self.is_empty
    }

    pub fn root(&self) -> bool {
        self.is_root
    }

    // Only adds files if it contains entries
    pub fn add_file(&mut self, path: PathBuf) {
        if let Some(file) = StodoFile::from_file(path) {
            if !file.is_empty() {
                self.stodos.push(file);
                self.set_not_empty();
            }
        }
    }

    // This function adds a stodo file even if it does not have any entries
    pub fn force_add_file(&mut self, path: PathBuf) {
        if let Some(file) = StodoFile::from_file(path) {
            self.stodos.push(file);
            self.set_not_empty();
        }
    }
}

impl Hash for StodoDir {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.path.hash(state)
    }
}

impl PartialEq for StodoDir {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

// impl PartialOrd for StodoDir {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         Some(PathBuf::cmp(&self.path, &other.path))
//     }
// }

impl Eq for StodoDir {
}
