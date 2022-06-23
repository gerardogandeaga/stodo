/// TODO: Revisit the mutlithreaded approach

use std::thread::{self, JoinHandle};
use std::path::PathBuf;
use std::collections::{HashMap, LinkedList};
use super::StodoDir;

const LOWER_WORKLOAD_BOUND: usize = 100;

/// Represents a thread that will search a given list of files for stodos
struct StodoSearcher<'a> {
    // worker: thread::JoinHandle<()>,
    is_alive: bool,
    searchable: HashMap<&'a StodoDir, LinkedList<PathBuf>>,
    workload: usize,
}

impl<'a> StodoSearcher<'a> {

    pub fn new() -> Self {
        // let handler: JoinHandle<()> = thread::spawn(|| {
        //     println!("Hello from thread: {:?}!", thread::current().id());
        //     while self.is_alive {
        //     }
        // });

        Self {
            // worker: handler,
            is_alive: true,
            searchable: HashMap::new(),
            workload: 0,
        }
    }

    pub fn delegate(&mut self, dir: &'a StodoDir, file: PathBuf) {
        // create a new entry in the searchable table
        if !self.searchable.contains_key(dir) {
            self.searchable.insert(dir, LinkedList::new());
        }
        else {
            self.searchable.get_mut(dir).unwrap().push_back(file);
        }

        self.workload += 1;
    }

    pub fn workload(&self) -> usize {
        self.workload
    }

    pub fn end(&mut self) {
        self.is_alive = false;
        // self.worker.join();
    }
}

/// Manages and distributes work to the worker threads
struct StodoSearchPool<'a> {
    searchers: Vec<StodoSearcher<'a>>
}

impl<'a> StodoSearchPool<'a> {

    pub fn new() -> Self {
        // initialize some searcher threads
        Self { searchers: vec![StodoSearcher::new()]}
    }

    pub fn add_file(&mut self,dir: &'a StodoDir, file: PathBuf) {
        // find the next available worker thread
        if let Some(w) = self.searchers.iter_mut().find(|w| w.workload() < LOWER_WORKLOAD_BOUND) {
            w.delegate(dir, file);
        }
        // assign to the worker that is doing the least amount of work
        else {
            let w: &mut StodoSearcher = self.searchers.iter_mut().min_by(|a, b| usize::cmp(&a.workload(), &b.workload())).unwrap();
            w.delegate(dir, file);
        }
    }
}




