mod cli;
mod core;
mod display;

use std::io;
use std::cmp::Ordering;
use std::path::Path;
use std::time;
use crossterm::{Result};
use ignore::WalkBuilder;

enum DirEntry {
    X(walkdir::DirEntry),
    Y(ignore::DirEntry),
}

impl DirEntry {
    fn path(&self) -> &Path {
        match *self {
            DirEntry::X(ref x) => x.path(),
            DirEntry::Y(ref y) => y.path(),
        }
    }
}

fn main() -> Result<()> {
    // STODO ----------------------------------------------------------------------------------------
    // get program configuration
    let stodo_config = cli::run_config();

    // generate the stodo trees
    let t = time::Instant::now();
    let stodo_forest: core::StodoForest = core::StodoWalker::new(&stodo_config).build_stodo_forest();
    // let stodo_trees: core::StodoForest = core::build_stodo_forest(stodo_config.src_paths, stodo_config.recursive);
    println!("forest generation time: {}", t.elapsed().as_millis());
    // build the display trees
    let t = time::Instant::now();
    let forest_str = display::builder::DisplayForestBuilder::compile(&stodo_forest);
    println!("display generation time: {}", t.elapsed().as_millis());
    // display the trees
    let mut stdout = io::stdout();
    cli::run(&mut stdout, forest_str)?;
    Ok(())
    // ----------------------------------------------------------------------------------------------
}


