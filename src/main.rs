mod cli;
mod core;
mod display;

use std::io;
use crossterm::{Result};

fn main() -> Result<()> {
    // STODO ----------------------------------------------------------------------------------------
    // get program configuration
    let stodo_config = cli::run_config();

    if !stodo_config.runnable() {
        return Ok(());
    }

    // generate the stodo trees
    let stodo_forest: core::StodoForest = core::StodoWalker::new(&stodo_config).build_stodo_forest();
    // build the display trees
    let stodo_output = display::builder::DisplayForestBuilder::compile(&stodo_forest);
    // display the trees
    let mut stdout = io::stdout();
    cli::run(&mut stdout, stodo_output)?;
    Ok(())
    // ----------------------------------------------------------------------------------------------
}


