use clap::{Arg, ArgMatches, Command, Values};

pub fn clap_args() -> ArgMatches {
    // TODO: Add a max recursion depth
    Command::new("stodo")
        .version("0.0.0")
        .author("gerardo gandeaga")
        .about("TODO cli manager")
        .arg(Arg::new("paths")
            .index(1)
            .multiple_values(true)
            // .short('p')
            // .long("paths")
            .value_name("PATH")
            .default_value(".")
            .help("Source paths"))
        .arg(Arg::new("recursive")
            .multiple_values(false)
            .short('r')
            .long("recursive")
            .help("Recursively search directories for TODOs"))
        .get_matches()
}