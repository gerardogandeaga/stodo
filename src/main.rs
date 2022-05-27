mod cli;
mod core;
mod display;

use std::io;
use std::time;
use crossterm::{Result};
use ignore::WalkBuilder;

fn main() -> Result<()> {
    // STODO ----------------------------------------------------------------------------------------
    // get program configuration
    let stodo_config = cli::run_config();

    // for res in WalkBuilder::new("./").git_ignore(true).build() {
    //     match res {
    //         Ok(entry) => {
    //             if entry.path().is_dir() {
    //                 println!("{}", entry.path().display())
    //             }
    //         },
    //         Err(err) => println!("Error: {}", err),
    //     }
    // }

    // generate the stodo trees
    let t = time::Instant::now();
    let stodo_trees: core::StodoForest = core::build_stodo_forest(stodo_config.src_paths, stodo_config.recursive);
    println!("forest generation time: {}", t.elapsed().as_millis());
    // build the display trees
    let t = time::Instant::now();
    let forest_str = display::builder::DisplayForestBuilder::compile(&stodo_trees);
    println!("display generation time: {}", t.elapsed().as_millis());
    // display the trees
    let mut stdout = io::stdout();
    cli::run(&mut stdout, forest_str)?;
    Ok(())
    // ----------------------------------------------------------------------------------------------
}


