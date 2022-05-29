/*
 * TODO: file path error handling. Right incorrect paths get ignored.
 */
pub mod file;
pub mod dir;
pub mod entry;

use std::path::{PathBuf, Path};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use petgraph::EdgeDirection;
use petgraph::graph::{Graph, NodeIndex};
use petgraph::visit::{NodeIndexable, DfsPostOrder};
use ignore::{WalkBuilder, Walk};

use crate::cli::stodo_args::CliConfig;

pub use {
    file::StodoFile,
    dir::StodoDir,
    entry::StodoEntry,
};

pub type StodoTree = Graph<StodoDir, i32, petgraph::Directed>;
pub type StodoForest = Vec<StodoTree>;

pub struct StodoWalker {
    paths: Vec<StodoPathEntry>,
    recursive: bool,
    search_all: bool,
}

impl StodoWalker {
    pub fn new(config: &CliConfig) -> Self {
        StodoWalker {
            paths: Self::process_path_entries(&config.paths()),
            recursive: config.recursive(),
            search_all: config.search_all(),
        }
    }

    /// Filters and groups paths if required. Only groups files paths that are under the same directory.
    /// Multiple declarations of the same directory will be processed twice.
    fn process_path_entries(paths: &Vec<String>) -> Vec<StodoPathEntry> {
        let paths: Vec<PathBuf> = paths.iter().map(|path| PathBuf::from(path)).collect();

        let mut entries_with_files: Vec<StodoPathEntry> = vec![];
        let mut entries_only_dirs: Vec<StodoPathEntry> = vec![];

        paths.iter().enumerate().for_each(|(order, path)| {
            if path.is_dir() { 
                entries_only_dirs.push(StodoPathEntry::new(PathBuf::from(path), order));
            }
            else 
            if path.is_file() {
                let mut directory: PathBuf = PathBuf::from(path);
                directory.pop();
                assert!(directory.is_dir(), "Parent must be a directory!");
                let mut dir_entry: StodoPathEntry = StodoPathEntry::new(directory, order);

                if let Some(entry) = entries_with_files.iter_mut().find(|e| e.eq(&&dir_entry)) {
                    entry.add_file(PathBuf::from(path));
                }
                else  {
                    dir_entry.add_file(PathBuf::from(path));
                    entries_with_files.push(dir_entry);
                }
            }
        });

        entries_only_dirs.append(&mut entries_with_files);
        entries_only_dirs.sort_by(|a,b| a.cmp(b));
        entries_only_dirs
    }
    
    /// Visits the nodes in a bottom-up fashion and trims the dangling leafs
    fn trim_empty_branches(tree: &mut StodoTree) {
        let root: NodeIndex = tree.from_index(0);
        let mut dfs = DfsPostOrder::new(&*tree, root);

        while let Some(node) = dfs.next(&*tree) {
            let is_leaf = tree.edges(node).count() == 0;
            
            if let Some(dir) =  tree.node_weight(node) {
                // remove the edge between the parent and the current node
                if is_leaf && dir.is_empty() {
                    // this walker should only need to iterate over a single edge
                    let mut walker = tree.neighbors_directed(node, EdgeDirection::Incoming).detach();
                    let mut removed_edge = false;
                    while let Some(incoming_edge) = walker.next_edge(tree) { 
                        assert_eq!(removed_edge, false, "This node seems to have more than a single parent...");
                        tree.remove_edge(incoming_edge); 
                        removed_edge = true;
                    }
                }
            }
        }
    }
}

impl StodoWalker {

    pub fn build_stodo_forest(&self) -> StodoForest {
        let mut trees: StodoForest = self.paths.iter()
            .map(|path_entry| self.build_stodo_tree(path_entry))
            .collect();

        trees.iter_mut()
            .for_each(|tree| {
                Self::trim_empty_branches(tree);
            });
    
        trees
    }

    fn build_stodo_tree(&self, path_entry: &StodoPathEntry) -> StodoTree {
        let is_recursive: bool = self.recursive && path_entry.search_dir();
        let specific_files: bool = !path_entry.search_dir();
        let directory_walker: Walk = self.directory_walker(&path_entry.dir, is_recursive);
        let mut stodo_tree: StodoTree = Graph::new();
        let mut parent_stack: Vec<NodeIndex> = vec![];
        let mut depth_stack: Vec<usize> = vec![];
        for res in directory_walker {
            match res {
                Ok(entry) => {
                    let mut n: usize = depth_stack.len();
                    while n > 0 && depth_stack[n-1] >= entry.depth() {
                        depth_stack.pop();
                        parent_stack.pop();
                        n -= 1;
                    }

                    let path: &Path = entry.path();
                    // Directory
                    if path.is_dir() {
                        // add directory to the tree
                        let curr_node: NodeIndex = stodo_tree.add_node(
                                StodoDir::new(path.to_path_buf(), parent_stack.is_empty())
                        );
                        // create an edge between parent and child node. Unless the node is the root obviously
                        if !parent_stack.is_empty() {
                            let parent_node = parent_stack.last().unwrap();
                            stodo_tree.add_edge(parent_node.clone(), curr_node, 1);
                        }
                        parent_stack.push(curr_node);
                        depth_stack.push(entry.depth());
                    }
                    else
                    // File
                    if path.is_file() {
                        let path_buf: PathBuf = path.to_path_buf();

                        if specific_files && !path_entry.has_file(&path_buf) {
                            continue;
                        }

                        if let Some(curr_node) = parent_stack.last() {
                            let curr_dir: &mut StodoDir = stodo_tree.node_weight_mut(curr_node.clone()).unwrap();

                            if specific_files {
                                curr_dir.force_add_file(path_buf);
                            }
                            else {
                                curr_dir.add_file(path_buf);
                            }
                        }
                    }
                },
                Err(err) => println!("Error: {}", err),
            }
        }
        stodo_tree
    }

    fn directory_walker(&self, dir: &PathBuf, is_recursive: bool) -> Walk {
        println!("use ignore? {}", !self.search_all);
        // TODO: get the ignore walker working
        WalkBuilder::new(PathBuf::from(dir))
            .sort_by_file_path(Path::cmp)
            .follow_links(false)
            .git_ignore(!self.search_all)
            .max_depth(if is_recursive {None} else {Some(1)})
            .build()
    }
}

/// Processed input paths from user input
struct StodoPathEntry {
    dir: PathBuf,
    files: HashMap<PathBuf, bool>,
    search_dir: bool,
    order: usize,
}

impl StodoPathEntry {
    pub fn new(dir: PathBuf, order: usize) -> Self {
        StodoPathEntry {
            dir,
            files: HashMap::new(),
            search_dir: true,
            order
        }
    }

    pub fn add_file(&mut self, file: PathBuf) {
        self.search_dir = false;

        if !self.files.contains_key(&file) {
            self.files.insert(file, true);
        }
    }

    pub fn has_file(&self, file: &PathBuf) -> bool {
        self.files.contains_key(file)
    }

    pub fn search_dir(&self) -> bool {
        self.search_dir
    }
}

impl PartialEq for StodoPathEntry {
    fn eq(&self, other: &Self) -> bool {
        self.dir == other.dir
    }
}

impl Hash for StodoPathEntry {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.dir.hash(state)
    }
}

impl Ord for StodoPathEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        usize::cmp(&self.order, &other.order)
    }
}

impl PartialOrd for StodoPathEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(usize::cmp(&self.order, &other.order))
    }
}

impl Eq for StodoPathEntry {}
