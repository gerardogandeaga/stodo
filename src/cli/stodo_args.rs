use clap::{ArgMatches};

pub struct CliConfig {
    pub src_paths: Vec<String>,
    pub recursive: bool
}

impl CliConfig {

    pub fn from(arg_matches: &ArgMatches) -> CliConfig {

        CliConfig {
            src_paths: arg_matches.values_of("paths").unwrap().map(String::from).collect(),
            recursive: arg_matches.is_present("recursive"),
        }
    }
}

