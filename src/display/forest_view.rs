use termtree;
use termtree::Tree;
use crate::core::{dir::StodoDir, StodoForest};

use super::{LineToken, builder, viewable::{self, Viewable}};
use builder::Visitor;
use super::displayable::Displayable;


pub struct ForestView {
    display_trees: Vec<termtree::Tree<String>>,
    compiled_str: Option<String>,
    tokens: Vec<LineToken>,
}

impl Viewable for ForestView {
    fn compile(&mut self, stodo_forest: &StodoForest) -> Option<String> {
        self.build_forest(stodo_forest);

        let mut s = String::from("");
        let n_trees: usize = self.display_trees.len();
        self.display_trees.iter().enumerate().for_each(|(i, t)| {
            s.push_str(t.to_string().as_str());
            if i < n_trees - 1 {
                s.push('\n');
            }
        });
        // self.compiled_str = Some(s);

        Some(s)
    }
}

impl ForestView {

    pub fn new() -> Self {
        Self { 
            display_trees: vec![], 
            compiled_str: None,
            tokens: vec![]
        }
    }

    pub fn to_string(&mut self) -> String {
        let s = self.compiled_str.clone().unwrap();
        s
    }

    fn build_forest(&mut self, stodo_trees: &StodoForest) {
        // build and print the todos in a tree structure
        let stodo_trees = self.build_displayable_forest(stodo_trees);
        self.display_trees.extend(stodo_trees);
    }

    /// traverses the stodo tree and builds a tree view
    fn build_displayable_forest(&mut self , stodo_trees: &StodoForest) -> Vec<termtree::Tree<String>> {
        let n_trees = stodo_trees.len();
        let mut display_trees: Vec<termtree::Tree<String>> = vec![];
        let mut subtree_stack: Vec<termtree::Tree<String>> = vec![];

        builder::DisplayBuilder::traverse(stodo_trees, 
            |visitor| {
                match visitor {
                    Visitor::Root(dir) => {
                        // wipe the tree stack on new tree
                        subtree_stack.clear();

                        let mut new_tree = termtree::Tree::new(dir.to_displayable());
                        self.add_line_token(LineToken::RootDir);
                        self.add_file_leaf(dir, &mut new_tree);
                        subtree_stack.push(new_tree);
                    },
                    Visitor::Node(dir, is_leaf, backtracked) => {
                        // performed some backtracking - "traversed" up the tree
                        for _ in 0..backtracked {
                            let sub_tree = subtree_stack.pop().unwrap();
                            subtree_stack.last_mut().unwrap().push(sub_tree);
                        }

                        // new tree node
                        let mut dir_branch: Tree<String> = termtree::Tree::new(dir.to_displayable());
                        self.add_line_token(LineToken::Dir);

                        // make either directory entry or file entry
                        if !dir.empty_stodos() {
                            self.add_file_leaf(dir, &mut dir_branch);
                        }

                        // simply push the leaf as a child
                        if is_leaf {
                            subtree_stack.last_mut().unwrap().push(dir_branch); 
                        }
                        // new branch to traverse down
                        else {
                            subtree_stack.push(dir_branch);
                        }
                    },
                    Visitor::End(i) => {
                        while subtree_stack.len() > 1 {
                            let sub_tree = subtree_stack.pop().unwrap();
                            subtree_stack.last_mut().unwrap().push(sub_tree);
                        }
                
                        assert!(subtree_stack.len() == 1, "This last item on the termtree stack must be the root!");
                        display_trees.push(subtree_stack.pop().unwrap());
            
                        // add empty line token for the extra spacing
                        if i < n_trees - 1 {
                            self.add_line_token(LineToken::Div);
                        }
                    },
                };
            }
        );

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
