use std::path::{PathBuf};
use std::fs;
use ansi_term::Colour;
use super::StodoFile;

/*
Represents a directory with files that have valid todo strings in them
TODO: only keep the relative path of the directories below
 */
#[derive(Debug)]
pub struct StodoDir {
    abs_path: PathBuf,       // absolute path of the directory
    in_path: PathBuf,        // path that the user entered, could be relative, could be absolute
    stodos: Vec<StodoFile>,  // todos in this directory
    specific_stodo_files: Vec<PathBuf>,
    search_all: bool,        // whether or not we want to search the entire directory
    depth: u32,
}

#[allow(dead_code)]
impl StodoDir {

    fn new() -> Self {
        Self {
            abs_path: PathBuf::from("./"),
            in_path: PathBuf::from("./"),
            stodos: vec![],
            specific_stodo_files: vec![],
            search_all: false,
            depth: 0,
        }
    }

    pub fn from_root(path: &PathBuf, specific_files: &Vec<String>) -> Option<Self> {
        if !path.is_absolute() || !path.is_dir() {
            return None
        }

        let mut files: Vec<PathBuf> = specific_files.iter()
            .map(|p| PathBuf::from(p))
            .collect();

        let i = files.iter().position(|p| fs::canonicalize(p).unwrap().as_path().eq(path))
            .map(|x| x as i32)
            .unwrap_or_else(|| -1);
        let search_all = i >= 0;

        if search_all { files.remove(i as usize); }

        let stodo_dir = Self::new()
            .with_path(path)
            .with_depth(0)
            .with_specific_files(files)
            .with_search_all(search_all);

        Some(stodo_dir)
    }

    /*
    create a new stodo dir from a path buffer
    */
    fn from_child(path: &PathBuf, depth: u32) -> Option<Self> {
        if !path.is_absolute() || !path.is_dir() {
            return None
        }

        let stodo_dir = Self::new()
            .with_path(path)
            .with_depth(depth)
            .with_search_all(true);

        Some(stodo_dir)
    }
}

impl StodoDir {

    /// Returns a list of sub stodo directories
    pub fn sub_dirs(&self) -> Vec<Self> {
        let dir_path = PathBuf::from(&self.abs_path);
        let dir_entries = fs::read_dir(&dir_path).unwrap();

        let mut sub_dirs: Vec<Self> = vec![];
        for entry in dir_entries {
            let path_buf = entry.unwrap().path();
            if path_buf.is_dir() {
                sub_dirs.push(Self::from_child(&path_buf, self.depth + 1).unwrap());
            }
        }

        sub_dirs
    }

    /// Find and add the stodos for the directory
    /// TODO:
    ///    - Parallelize
    ///    - Compress nodes that do not have stodos (middle nodes)
    pub fn populate_stodos(&mut self) {
        if self.search_all {
            let stodos: Option<Vec<StodoFile>> = StodoFile::from_dir(&self.abs_path);

            if stodos.is_some() {
                self.stodos.extend(stodos.unwrap());
            }
        }
        else {
            self.specific_stodo_files.iter()
                .map(|ps| StodoFile::from_file(PathBuf::from(ps)))
                .for_each(|stodo| {
                    if stodo.is_some() {
                        self.stodos.push(stodo.unwrap());
                    }
                });
        }
        // TODO: what should happen if there are no stodos in this directory?
        if self.stodos().is_empty() {
        }
    }

    fn with_path(mut self, path: &PathBuf) -> Self {
        self.in_path = PathBuf::from(path);
        self.abs_path = fs::canonicalize(path).unwrap().to_path_buf();
        self
    }

    fn with_depth(mut self, depth: u32) -> Self {
        self.depth = depth;
        self
    }

    fn with_specific_files(mut self, files: Vec<PathBuf>) -> Self {
        self.specific_stodo_files.extend(files);
        self
    }

    fn with_search_all(mut self, search_all: bool) -> Self {
        self.search_all = search_all;
        self
    }

    pub fn stodos(&self) -> &Vec<StodoFile> {
        &self.stodos
    }

    pub fn in_path(&self) -> &PathBuf {
        &self.in_path
    }

    pub fn depth(&self) -> u32 {
        self.depth
    }

    pub fn is_empty(&self) -> bool {
        self.stodos.is_empty()
    }
}
