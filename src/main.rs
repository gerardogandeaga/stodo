mod cli;
mod stodo_tree;
mod display;

extern crate petgraph;

// use petgraph::dot::Dot;

fn main() {
    let stodo_config = cli::run_config();

    // let stodo_trees = stodo_tree::build_stodo_trees(stodo_config.src_paths, stodo_config.recursive);

    // display::print_tree::display_stodo_tree(&stodo_trees);

    display::builder::test();


    // stodo_trees.into_iter()
    //     .for_each(|tree| println!("{}", Dot::new(&tree)));
}


