use clap::{ArgMatches};

pub struct CliConfig {
    src_paths: Vec<String>,
    recursive: bool,
    search_all: bool,
}

impl CliConfig {

    pub fn from(arg_matches: &ArgMatches) -> CliConfig {

        CliConfig {
            src_paths: arg_matches.values_of("paths").unwrap().map(String::from).collect(),
            recursive: arg_matches.is_present("recursive"),
            search_all: arg_matches.is_present("all")
        }
    }

    pub fn paths(&self) -> &Vec<String> {
        &self.src_paths
    }

    pub fn recursive(&self) -> bool {
        self.recursive
    }

    pub fn search_all(&self) -> bool {
        self.search_all
    }
}

