/*
 * TODO: create a custom tree printer
 */

use super::builder;
use std::path::PathBuf;
use ptree;
use petgraph::graph::{Graph, NodeIndex};
use petgraph::visit::{Dfs, NodeIndexable};

use crate::stodo_tree::{StodoDir};

pub fn display_stodo_tree(stodo_trees: &Vec<Graph<StodoDir, i32>>) {
    println!("TODOs");

    // build and print the todos in a tree structure
    let mut stodo_trees = build_display_tree(stodo_trees);
    for stodo_tree in stodo_trees.iter_mut() {
        let displayable = &stodo_tree.build();
        ptree::print_tree(displayable).ok();
    }
}

fn build_display_tree(stodo_trees: &Vec<Graph<StodoDir, i32>>) -> Vec<ptree::TreeBuilder> {
    let mut display_trees: Vec<ptree::TreeBuilder> = vec![];

    for tree in stodo_trees.iter() {
        let root: NodeIndex = tree.from_index(0);
        let mut dfs = Dfs::new(&tree, root);

        let mut display_tree = ptree::TreeBuilder::new(
            basename_str(tree.node_weight(root).unwrap().in_path()));
        build_display_branch(tree.node_weight(root).unwrap(), &mut display_tree);
        dfs.next(&tree);

        let mut level_stack: Vec<usize> = vec![];
        while let Some(node) = dfs.next(&tree) {
            let stodo_dir: &StodoDir = tree.node_weight(node).unwrap();
            let n_edges: usize = tree.edges(node).count();
            let mut n: usize = level_stack.len();

            while n > 0 && level_stack[n-1] == 0 {
                level_stack.pop();
                display_tree.end_child();
                n -= 1;
            }

            if !level_stack.is_empty() {
                let i: usize = level_stack.len() - 1;
                if level_stack[i] > 0 {
                    level_stack[i] -= 1;
                }
            }

            display_tree.begin_child(basename_str(stodo_dir.in_path()));
            build_display_branch(stodo_dir, &mut display_tree);

            if n_edges > 0 {
                level_stack.push(n_edges);
            }
            else {
                display_tree.end_child();
            }
        }

        display_trees.push(display_tree);
    }

    display_trees
}

/// add the files with todos and their stodo strings
fn build_display_branch(todo_dir: &StodoDir, tree: &mut ptree::TreeBuilder) {
    for stodo in todo_dir.stodos().iter() {
        // make the filename a parent
        tree.begin_child(basename_str(&stodo.path));
        for todo_str in stodo.str_todos.iter() {
            tree.add_empty_child(String::from(todo_str));
        }
        tree.end_child();
    }
}

fn basename_str(path: &PathBuf) -> String {
    let mut basename = String::from(path.file_name().unwrap_or_default().to_str().unwrap());

    if path.is_dir() {
        basename.push('/');
    }

    basename
}
