pub mod stodo;
pub mod stodo_dir;

use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;
use petgraph::EdgeDirection;
use petgraph::graph::{Graph, NodeIndex};
use petgraph::visit::{NodeIndexable, DfsPostOrder};

pub use {
    stodo::StodoFile,
    stodo_dir::StodoDir
};

pub type StodoTree = Graph<StodoDir, i32, petgraph::Directed>;
pub type StodoForest = Vec<StodoTree>;

pub fn build_stodo_trees(src_paths: Vec<String>, recursive: bool) -> StodoForest {
    // let t = std::time::Instant::now();
    let dir_mappings = unique_dir_paths(&src_paths);
    let mut trees: StodoForest = dir_mappings.into_iter()
        .map(|path| build_stodo_tree(&path, recursive))
        .collect();
    // println!("build dir tree: {}", t.elapsed().as_secs_f32());

    // populate the directories with the stodos
    // TODO: parallelize?
    // let T = std::time::Instant::now();

    // for (tree, ) in trees.iter_mut().enumerate() {

    // }
    trees.iter_mut()
        .for_each(|tree| {
            trim_empty_branches(tree);
        });
    // println!("populate directories: {}", T.elapsed().as_secs_f32());

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
            let sub_dirs: Vec<StodoDir> = stodo_tree.node_weight_mut(node).unwrap().sub_dirs();
            let children: Vec<NodeIndex> = sub_dirs.into_iter()
            .map(|x| {
                let child = stodo_tree.add_node(x);
                stodo_tree.add_edge(node, child, 1);
                child
            })
            .collect();

            // populate the directory with stodos
            stodo_tree.node_weight_mut(node).unwrap().populate_stodos();

            traverse.extend(children);
        }
    }

    // TODO: ensure tree validity
    // stodo_tree.c

    stodo_tree
}

/// Returns a mapping between the paths and their parent directory or itself if it's a directory
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

/// Visits the nodes in a bottom-up fashion and trims the dangling leafs
fn trim_empty_branches(tree: &mut StodoTree) {
    let root: NodeIndex = tree.from_index(0);
    let mut dfs = DfsPostOrder::new(&*tree, root);

    // println!("{}", tree.node_count());

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

    // tree.filter_map(|x,y| {
    //     let is_single = (tree.edges_directed(x, EdgeDirection::Incoming).count() + tree.edges_directed(x, EdgeDirection::Outgoing).count()) == 0;
    //     if is_single {
    //         None
    //     }
    //     else {
    //         Some(y)
    //     }
    // }, 
    // |_,y| Some(y));

    // TODO: remove the nodes that do not have any edges
    // println!("{}", tree.node_count());
}
