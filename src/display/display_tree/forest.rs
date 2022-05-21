/*
 * TODO: create a custom tree printer
 */

use std::path::PathBuf;
use termtree;
use petgraph::graph::{NodeIndex};
use petgraph::visit::{Dfs, NodeIndexable};
use crate::stodo_tree::{stodo_dir::StodoDir, StodoForest};

use super::builder::LineToken;

pub struct Forest {
    display_trees: Vec<termtree::Tree<String>>,
    compiled_str: Option<String>,
    tokens: Vec<LineToken>,
}

impl Forest {

    pub fn new() -> Self {
        Self { 
            display_trees: vec![], 
            compiled_str: None,
            tokens: vec![]
        }
    }

    pub fn compile(&mut self, stodo_trees: &StodoForest) {

        self.build_forest(stodo_trees);

        let mut s = String::from("");
        self.display_trees.iter().for_each(|t| s.push_str(t.to_string().as_str()));
        self.compiled_str = Some(s);
    }

    pub fn to_string(&mut self) -> String {
        let s = self.compiled_str.clone().unwrap();
        s
    }

    fn build_forest(&mut self, stodo_trees: &StodoForest) {
        // build and print the todos in a tree structure
        let stodo_trees = self.build_display_tree(stodo_trees);
        self.display_trees.extend(stodo_trees);
    }
    
    fn build_display_tree(&mut self , stodo_trees: &StodoForest) -> Vec<termtree::Tree<String>> 
    {
        let mut display_trees: Vec<termtree::Tree<String>> = vec![];
    
        for tree in stodo_trees.iter() {
            let root: NodeIndex = tree.from_index(0);
            let mut dfs = Dfs::new(&tree, root);
            
            let mut tt_root = termtree::Tree::new(
                basename_str(tree.node_weight(root).unwrap().in_path())
            );
            self.add_line_token(LineToken::Dir);
            self.add_file_leaf(tree.node_weight(root).unwrap(), &mut tt_root);
            
            dfs.next(&tree);
    
            let mut level_stack: Vec<usize> = vec![];
            let mut tree_stack: Vec<termtree::Tree<String>> = vec![tt_root];
            while let Some(node) = dfs.next(&tree) {
                let stodo_dir: &StodoDir = tree.node_weight(node).unwrap();
                let n_edges: usize = tree.edges(node).count();
                let mut n: usize = level_stack.len();
    
                while n > 0 && level_stack[n-1] == 0 {
                    level_stack.pop();
                    let sub_tree: termtree::Tree<String> = tree_stack.pop().unwrap();
                    tree_stack.last_mut().unwrap().push(sub_tree);
                    n -= 1;
                }
    
                if !level_stack.is_empty() {
                    let i: usize = level_stack.len() - 1;
                    if level_stack[i] > 0 {
                        level_stack[i] -= 1;
                    }
                }
    
                // make a directory branch
                let mut dir_branch: termtree::Tree<String> = termtree::Tree::new(basename_str(stodo_dir.in_path()));
                self.add_line_token(LineToken::Dir);
                self.add_file_leaf(stodo_dir, &mut dir_branch);
    
                if n_edges > 0 {
                    level_stack.push(n_edges);
                    tree_stack.push(dir_branch);
                }
                else {
                    tree_stack.last_mut().unwrap().push(dir_branch); // leaf directory
                }
            }
    
            while tree_stack.len() > 1 {
                let sub_tree: termtree::Tree<String> = tree_stack.pop().unwrap();
                tree_stack.last_mut().unwrap().push(sub_tree);
            }
    
            assert!(tree_stack.len() == 1, "This last item on the termtree stack must be the root!");
            display_trees.push(tree_stack.pop().unwrap());
        }
    
        display_trees
    }
    
    /// add the files with todos and their stodo strings
    fn add_file_leaf(
        &mut self,
        todo_dir: &StodoDir, 
        tree: &mut termtree::Tree<String>)
    {
        for stodo in todo_dir.stodos().iter() {
            self.add_line_token(LineToken::File);

            // make the filename a parent
            let f_branch = termtree::Tree::new(basename_str(stodo.file_path()))
                .with_leaves(stodo.stodo_entries().iter()
                    .map(|entry| {
                        self.add_line_token(LineToken::Stodo(entry.line_number()));
                        // dt_builder.add_stodo(entry);
                        termtree::Tree::new(entry.stodo_string())
                    })
                );
            tree.push(f_branch);
        }
    }

    fn add_line_token(&mut self, token: LineToken) {
        self.tokens.push(token);
    }

    pub fn line_tokens(&self) -> &Vec<LineToken> {
        &self.tokens
    }
}


fn basename_str(path: &PathBuf) -> String {

    let mut basename = String::from(path.file_name().unwrap_or_default().to_str().unwrap());

    if path.is_dir() {
        basename.push('/');
    }

    basename
}
