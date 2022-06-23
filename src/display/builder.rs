// TODO: refactor this file into display.rs
use super::gutter::Gutter;
use super::forest_view::ForestView;
use petgraph::graph::{NodeIndex};
use petgraph::visit::{Dfs, NodeIndexable};
use crate::core::{StodoForest, StodoDir, StodoTree};
use crate::display::viewable::Viewable;
use super::{LineToken, StodoOutput};

pub enum Visitor<'a> {
    Root(&'a StodoDir),
    Node(&'a StodoDir, bool, usize),
    End(usize),
}

pub struct DisplayBuilder {
    gutter: Gutter,
    forest: ForestView,
}

impl DisplayBuilder {

    fn new() -> Self {
        Self {
            gutter: Gutter::new(),
            forest: ForestView::new(),
        }
    }

    pub fn compile(stodo_forest: &StodoForest) -> StodoOutput {
        let mut display_builder: DisplayBuilder = Self::new();
        
        // generate the compiled string which also creates the line tokens! then we can create the gutter
        let forest_str: String = display_builder.forest.compile(stodo_forest).unwrap();
        let gutter_str: String = display_builder.gutter.compile(display_builder.forest.line_tokens());
        
        let mut display_str: String = String::with_capacity(gutter_str.len() + forest_str.len());
        
        gutter_str.split("\n").zip(forest_str.split("\n"))
            .for_each(|(g, f)| {
                display_str.push_str(g);
                display_str.push_str(f);
                display_str.push('\n');
            });

        let tokens: Vec<LineToken> = display_builder.forest.line_tokens().iter().map(|lt| lt.clone()).collect();
        let lines: Vec<String> = display_str.trim_end().split("\n").map(|l| String::from(l)).collect();
        
        assert!(tokens.len() == lines.len(), "Number of tokens must equal the number of lines!");
        let output: StodoOutput = StodoOutput { 
            line_tokens: tokens,
            out_strings: lines
        };

        output
    }

    /// Does a dfs traversal of the stodo forest
    pub fn traverse<V>(stodo_forest: &StodoForest, mut on_visit: V)
        where 
            V: FnMut(Visitor) -> ()
    {
        for (i, tree) in stodo_forest.iter().enumerate() {
            let root: NodeIndex = tree.from_index(0);
            let mut dfs = Dfs::new(&tree, root);

            // do something special for the root node
            on_visit(Visitor::Root(tree.node_weight(root).unwrap()));
            
            // keeps track of the number of nodes remaining at the current depth
            let mut n_children_stack: Vec<usize> = vec![];
            // traverse the tree
            dfs.next(&tree);
            while let Some(node) = dfs.next(&tree) {
                let stodo_dir: &StodoDir = tree.node_weight(node).unwrap();
                let n_children: usize = tree.edges(node).count(); // get number of children
                let mut n: usize = n_children_stack.len();
                
                // while all the nodes at the current depth has been explored then simply pop them off the depth stack
                // here we are tracking the levels we are back tracking by
                let mut backtracked: usize = 0;
                while n > 0 && n_children_stack[n-1] == 0 {
                    n_children_stack.pop();
                    backtracked += 1;
                    n -= 1;
                }

                // while we visit a node, decrement the number of children to visit counter
                if !n_children_stack.is_empty() {
                    let i: usize = n_children_stack.len() - 1;
                    if n_children_stack[i] > 0 {
                        n_children_stack[i] -= 1;
                    }
                }

                // node is visited
                let is_leaf: bool = n_children == 0;

                // signal that we wil continue to travel downwards
                if !is_leaf {
                    n_children_stack.push(n_children);
                }

                // on_visit(stodo_dir, is_leaf, false, backtracked);
                on_visit(Visitor::Node(stodo_dir, is_leaf, backtracked));
            }

            on_visit(Visitor::End(i));
        }
    }
}
