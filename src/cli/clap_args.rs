use clap::{Arg, ArgMatches, Command};

pub fn clap_args() -> ArgMatches {
    // TODO: Add a max recursion depth, use the .gitignore (deafult = true), view hidden files (default = false)
    Command::new("stodo")
        .version("0.2.0")
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
            .takes_value(false)
            .help("Recursively search directories for TODOs"))
        .arg(Arg::new("all")
            .multiple_values(false)
            .long("all")
            .takes_value(false)
            .help("Does not perform any filtering with .gitignore or .ignore files while searching. However, the program will filter out hidden files."))
        .get_matches()
}