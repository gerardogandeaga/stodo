mod cli;
mod stodo_forest;
mod display;

use std::io;
use std::time;
use crossterm::{Result};

fn main() -> Result<()> {
    // STODO ----------------------------------------------------------------------------------------
    // get program configuration
    let stodo_config = cli::run_config();
    // generate the stodo trees
    let t = time::Instant::now();
    let stodo_trees: stodo_forest::StodoForest = stodo_forest::build_stodo_trees(stodo_config.src_paths, stodo_config.recursive);
    println!("forest generation time: {}", t.elapsed().as_millis());
    // build the display trees
    let t = time::Instant::now();
    let forest_str = display::display_tree::builder::DisplayForestBuilder::compile(&stodo_trees);
    println!("display generation time: {}", t.elapsed().as_millis());
    // display the trees
    let mut stdout = io::stdout();
    cli::run(&mut stdout, forest_str);
    Ok(())
    // ----------------------------------------------------------------------------------------------
}


