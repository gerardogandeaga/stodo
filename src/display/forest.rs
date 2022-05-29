use termtree;
use petgraph::graph::{NodeIndex};
use petgraph::visit::{Dfs, NodeIndexable};
use crate::core::{dir::StodoDir, StodoForest};

use super::builder::LineToken;
use super::displayable::Displayable;

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
        let n_trees: usize = self.display_trees.len();
        self.display_trees.iter().enumerate().for_each(|(i, t)| {
            s.push_str(t.to_string().as_str());
            if i < n_trees - 1 {
                s.push('\n');
            }
        });
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
    
        let n_trees: usize = stodo_trees.len();
        for (i, tree) in stodo_trees.iter().enumerate() {
            let root: NodeIndex = tree.from_index(0);
            let mut dfs = Dfs::new(&tree, root);
            
            let mut tt_root = termtree::Tree::new(tree.node_weight(root).unwrap().to_displayable());
            self.add_line_token(LineToken::RootDir);
            self.add_file_leaf(tree.node_weight(root).unwrap(), &mut tt_root);
            
            dfs.next(&tree);
    
            let mut depth_stack: Vec<usize> = vec![];
            let mut tree_stack: Vec<termtree::Tree<String>> = vec![tt_root];
            while let Some(node) = dfs.next(&tree) {
                let stodo_dir: &StodoDir = tree.node_weight(node).unwrap();
                let n_edges: usize = tree.edges(node).count();
                let mut n: usize = depth_stack.len();
    
                while n > 0 && depth_stack[n-1] == 0 {
                    depth_stack.pop();
                    let sub_tree = tree_stack.pop().unwrap();
                    tree_stack.last_mut().unwrap().push(sub_tree);
                    n -= 1;
                }

                if !depth_stack.is_empty() {
                    let i: usize = depth_stack.len() - 1;
                    if depth_stack[i] > 0 {
                        depth_stack[i] -= 1;
                    }
                }
    
                let mut dir_branch = termtree::Tree::new(stodo_dir.to_displayable());
                self.add_line_token(LineToken::Dir);

                // make a directory branch
                if !stodo_dir.is_empty() {
                    self.add_file_leaf(stodo_dir, &mut dir_branch);
                }

                if n_edges > 0 {
                    depth_stack.push(n_edges);
                    tree_stack.push(dir_branch);
                }
                else {
                    tree_stack.last_mut().unwrap().push(dir_branch); // leaf directory
                }
            }
    
            while tree_stack.len() > 1 {
                let sub_tree = tree_stack.pop().unwrap();
                tree_stack.last_mut().unwrap().push(sub_tree);
            }
    
            assert!(tree_stack.len() == 1, "This last item on the termtree stack must be the root!");
            display_trees.push(tree_stack.pop().unwrap());

            // add empty line token for the extra spacing
            if i < n_trees - 1{
                self.add_line_token(LineToken::Empty);
            }
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
            let f_branch = termtree::Tree::new(stodo.to_displayable())
                .with_leaves(stodo.stodo_entries().iter()
                    .map(|entry| {
                        self.add_line_token(LineToken::Stodo(entry.line_number()));
                        // dt_builder.add_stodo(entry);
                        termtree::Tree::new(entry.to_displayable())
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
