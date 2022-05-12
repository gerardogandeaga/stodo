use std::fmt;
use std::borrow::{Borrow, BorrowMut};
use std::fmt::{Formatter, write};
use std::path::{Path, PathBuf};
use std::fs;
use std::fs::ReadDir;
use crate::stodo_items::StodoFile;

/*
Represents a directory with files that have valid todo strings in them
 */
#[derive(Debug)]
pub struct StodoDir {
    pub abs_path: PathBuf,       // absolute path of the directory
    pub in_path: PathBuf,        // path that the user entered, could be relative, could be absolute
    pub sub_dirs: Vec<StodoDir>, // subdirectories
    pub stodos: Vec<StodoFile>,   // todos in this directory
    pub search_all: bool,        // whether or not we want to search the entire directory
    depth: u32,
}

impl fmt::Display for StodoDir {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "d: {}, p: {}", self.depth, self.abs_path.display().to_string())
    }
}

impl StodoDir {

    fn new() -> StodoDir {
        StodoDir {
            abs_path: PathBuf::from("./"),
            in_path: PathBuf::from("./"),
            sub_dirs: vec![],
            stodos: vec![],
            search_all: false,
            depth: 0
        }
    }

    pub fn from_root(path: &PathBuf) -> Option<StodoDir> {
        if !path.is_absolute() || !path.is_dir() {
            return None
        }

        let mut stodo_dir = StodoDir::new();
        stodo_dir.with_path(path);
        stodo_dir.with_depth(0);

        Some(stodo_dir)
    }

    /*
    create a new stodo dir from a path buffer
    */
    fn from_child(path: &PathBuf, depth: u32) -> Option<StodoDir> {
        if !path.is_absolute() || !path.is_dir() {
            return None
        }

        let mut stodo_dir = StodoDir::new();
        stodo_dir.with_path(path);
        stodo_dir.with_depth(depth);

        Some(stodo_dir)
    }

    /*
     Returns a list of sub stodo directories
     */
    pub fn sub_dirs(&self) -> Vec<StodoDir> {
        let dir_path = PathBuf::from(&self.abs_path);
        let dir_entries = fs::read_dir(&dir_path).unwrap();

        let mut sub_dirs: Vec<StodoDir> = vec![];
        for entry in dir_entries {
            let path_buf = entry.unwrap().path();
            if path_buf.is_dir() {
                sub_dirs.push(StodoDir::from_child(&path_buf, self.depth + 1).unwrap());
            }
        }

        sub_dirs
    }

    /*
     Find and add the stodos for the directory
     TODO:
        - Parallelize
        - Compress nodes that do not have stodos (middle nodes)
     */
    pub fn populate_stodos(&mut self) {
        let stodos = StodoFile::from_dir(&self.abs_path);

        if stodos.is_some() {
            self.stodos.extend(stodos.unwrap());
        }
        // TODO: what should happen if there are no stodos in this directory?
    }

    fn compress() {
        // TODO(implement)
    }

    fn with_path(&mut self, path: &PathBuf) {
        self.in_path = PathBuf::from(path);
        self.abs_path = fs::canonicalize(path).unwrap().to_path_buf();
    }

    fn with_depth(&mut self, depth: u32) {
        self.depth = depth;
    }

    pub fn depth(&self) -> u32 {
        self.depth
    }

    pub fn in_path(&self) -> &PathBuf {
        &self.in_path
    }
}
