pub mod stodo;
pub mod stodo_dir;

use std::fs;
use std::path::PathBuf;
use petgraph::graph::{Graph, NodeIndex};
use petgraph::visit::{Dfs, NodeIndexable};
use crate::cli::stodo_args::CliConfig;

pub use {
    stodo::StodoFile,
    stodo_dir::StodoDir
};

pub fn build_stodo_trees(src_paths: Vec<String>, recursive: bool) -> Vec<Graph<StodoDir, i32, petgraph::Directed>> {
    let mut trees: Vec<Graph<StodoDir, i32, petgraph::Directed>> = src_paths.into_iter()
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

fn build_stodo_tree(src_path: &String, recursive: bool) -> Graph<StodoDir, i32, petgraph::Directed> {
    // get the root stodo directory so it can be the root of the tree
    let input_path_buff: PathBuf = PathBuf::from(src_path);
    let mut abs_path_buff: PathBuf = fs::canonicalize(&input_path_buff).unwrap();

    // need to convert src path to a directory path so the StodoDir object can be created
    if !abs_path_buff.is_dir() {
        abs_path_buff = abs_path_buff.parent().unwrap().to_path_buf();
        assert!(abs_path_buff.is_dir(), "this path must be a directory!");
    }

    // build the stodo tree
    let root_stodo_dir: StodoDir = StodoDir::from_root(&abs_path_buff).unwrap();

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
