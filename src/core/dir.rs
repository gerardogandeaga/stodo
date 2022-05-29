use std::path::PathBuf;
use super::StodoFile;

/*
Represents a directory with files that have valid todo strings in them
TODO: only keep the relative path of the directories below
 */
#[derive(Debug)]
pub struct StodoDir {
    path: PathBuf,          // path that the user entered, could be relative, could be absolute
    stodos: Vec<StodoFile>, // todos in this directory
    is_root: bool
}

#[allow(dead_code)]
impl StodoDir {

    pub fn new(path: PathBuf, is_root: bool) -> Self {
        Self {
            path,
            stodos: vec![],
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

    pub fn is_empty(&self) -> bool {
        self.stodos.is_empty()
    }

    pub fn is_root(&self) -> bool {
        self.is_root
    }

    // Only adds files if it contains entries
    pub fn add_file(&mut self, path: PathBuf) {
        if let Some(file) = StodoFile::from_file(path) {
            if !file.is_empty() {
                self.stodos.push(file);
            }
        }
    }

    // This function adds a stodo file even if it does not have any entries
    pub fn force_add_file(&mut self, path: PathBuf) {
        if let Some(file) = StodoFile::from_file(path) {
            self.stodos.push(file);
        }
    }
}
