mod cli;
mod core;
mod display;

use std::io;
use crossterm::Result;


use std::time;

fn main() -> Result<()> {
    // STODO ----------------------------------------------------------------------------------------
    // get program configuration
    let stodo_config = cli::run_config();
    
    if !stodo_config.runnable() {
        return Ok(());
    }
    
    let timer = time::Instant::now();
    // generate the stodo forest
    let stodo_forest: core::StodoForest = core::StodoWalker::new(&stodo_config).build_stodo_forest();
    println!("Time to create forest: {}ms - {}s", timer.elapsed().as_millis(), timer.elapsed().as_secs_f32()) ;
    
    // build the display trees
    let stodo_output = display::builder::DisplayBuilder::compile(&stodo_forest);
    // display the trees
    let mut stdout = io::stdout();
    cli::run(&mut stdout, stodo_output)?;
    Ok(())
    // ----------------------------------------------------------------------------------------------
}


