use clap::{Arg, ArgMatches, Command};

pub fn clap_args() -> ArgMatches {
    // TODO: Add a max recursion depth
    Command::new("stodo")
        .version("0.0.0")
        .author("gerardo gandeaga")
        .about("TODO cli manager")
        .arg(Arg::new("paths")
            .index(1)
            .multiple_values(true)
            .value_name("PATH")
            .default_value(".")
            .help("Source paths"))
        .arg(Arg::new("recursive")
            .multiple_values(false)
            .short('r')
            .long("recursive")
            .help("Recursively search directories for TODOs"))
        .arg(Arg::new("gitignore")
            .multiple_values(false)
            .long("gitignore")
            .help("Filter files found in .gitignore files"))
        .get_matches()
}