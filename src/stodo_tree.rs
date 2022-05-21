pub mod stodo;
pub mod stodo_dir;

use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;
use petgraph::graph::{Graph, NodeIndex};
use petgraph::visit::{Dfs, NodeIndexable};

pub type StodoTree = Graph<StodoDir, i32, petgraph::Directed>;
pub type StodoForest = Vec<StodoTree>;

pub use {
    stodo::StodoFile,
    stodo_dir::StodoDir
};

pub fn build_stodo_trees(src_paths: Vec<String>, recursive: bool) -> StodoForest {
    let dir_mappings = unique_dir_paths(&src_paths);

    let mut trees: StodoForest = dir_mappings.into_iter()
        .map(|path| build_stodo_tree(&path, recursive))
        .collect();

    // populate the directories with the stodos
    // TODO: parallelize?
    for tree in trees.iter_mut() {
        let root: NodeIndex = tree.from_index(0);
        let mut dfs = Dfs::new(&*tree, root);

        while let Some(node) = dfs.next(&*tree) {
            tree.node_weight_mut(node)
                .unwrap()
                .populate_stodos();
        }
    }

    trees
}

fn build_stodo_tree(info: &(PathBuf, Vec<String>), recursive: bool) -> StodoTree {
    let root_stodo_dir: StodoDir = StodoDir::from_root(&info.0, &info.1).unwrap();

    let mut stodo_tree: Graph<StodoDir, i32> = Graph::new();
    let root: NodeIndex = stodo_tree.add_node(root_stodo_dir);

    // need to recursively build the tree if recursive was specified
    if recursive {
        let mut traverse: Vec<NodeIndex> = vec![root];

        while !traverse.is_empty() {
            let node: NodeIndex = traverse.pop().unwrap();

            // expand node to get the sub directories which will be added to the graph as children
            let sub_dirs: Vec<StodoDir> = stodo_tree.node_weight(node).unwrap().sub_dirs();
            let children: Vec<_> = sub_dirs.into_iter()
                .map(|x| {
                    let child = stodo_tree.add_node(x);
                    stodo_tree.add_edge(node, child, 1);
                    child
                })
                .collect();

            traverse.extend(children);
        }
    }

    // TODO: ensure tree validity
    // stodo_tree.c

    stodo_tree
}

/*
 * Returns a mapping between the paths and their parent directory or itself if it's a directory
 */
fn unique_dir_paths(paths: &Vec<String>) -> HashMap<PathBuf, Vec<String>> {

    let mut mappings: HashMap<PathBuf, Vec<String>> = HashMap::new();

    for path in paths.iter() {
        let path_buff: PathBuf = PathBuf::from(path);
        let mut abs_path_buff: PathBuf = fs::canonicalize(&path_buff).unwrap();

        if !abs_path_buff.is_dir() {
            abs_path_buff = abs_path_buff.parent().unwrap().to_path_buf();
            assert!(abs_path_buff.is_dir(), "this path must be a directory!");
        }

        if mappings.contains_key(&abs_path_buff) {
            mappings.get_mut(&abs_path_buff).unwrap().push(String::from(path));
        }
        else {
            mappings.insert(abs_path_buff, vec![String::from(path)]);
        }
    }

    return mappings;
}
