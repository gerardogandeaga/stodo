mod cli;
mod stodo_items;
mod display;

extern crate petgraph;

use petgraph::dot::Dot;
use petgraph::graph::Graph;
use petgraph::visit::{Dfs, NodeIndexable, NodeRef};

fn main() {
    let stodo_config = cli::run_config();

    let stodo_trees = stodo_items::build_stodo_trees(stodo_config.src_paths, stodo_config.recursive);


    // let todos: Vec<_> = stodo_items::build_stodo_tree(stodo_config.src_paths, stodo_config.recursive);
    display::print_tree::display_stodo_tree(&stodo_trees);

    // let t: Graph<String, String> = tree();
    // let root = t.from_index(0);
    // let mut dfs = Dfs::new(&t, root);
    //
    // while let Some(node) = dfs.next(&t) {
    //     println!("{} {:?}",
    //         t.edge(), t.node_weight(node).unwrap());
    // }
    //
    stodo_trees.into_iter()
        .for_each(|tree| println!("{}", Dot::new(&tree)));
}

fn tree() -> Graph<String, String> {
    let mut t = Graph::new();
    let t_item1 = t.add_node("a".to_string());
    let t_item2 = t.add_node("b".to_string());
    let t_item3 = t.add_node("c".to_string());
    let t_item4 = t.add_node("d".to_string());
    let t_item5 = t.add_node("e".to_string());
    t.add_edge(t_item1, t_item2, "".to_string());
    t.add_edge(t_item1, t_item3, "".to_string());
    t.add_edge(t_item2, t_item4, "".to_string());
    t.add_edge(t_item2, t_item5, "".to_string());
    t
}


