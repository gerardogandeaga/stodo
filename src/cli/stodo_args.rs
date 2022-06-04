use std::path::PathBuf;

use clap::{ArgMatches};

pub struct CliConfig {
    src_paths: Vec<String>,
    recursive: bool,
    search_all: bool,
    runnable: bool,
}

impl CliConfig {

    pub fn from(arg_matches: &ArgMatches) -> CliConfig {
        // TODO: Check if the paths past in are valid paths
        let mut config = CliConfig {
            src_paths: arg_matches.values_of("paths").unwrap().map(String::from).collect(),
            recursive: arg_matches.is_present("recursive"),
            search_all: arg_matches.is_present("all"),
            runnable: true,
        };

        // process configs
        config.validate_paths();
        config.ensure_runnability();
        config
    }
}

impl CliConfig {

    /// checks and removes paths that aren't valid 
    fn validate_paths(&mut self) {
        assert!(!self.paths().is_empty(), "There should be at least 1 path");

        self.src_paths.retain(|path| {
            let exists = PathBuf::from(path).exists();
            if !exists {
                println!("stodo: {}: No such file or directory.", path);
            }
            exists
        });
    }

    fn ensure_runnability(&mut self) {
        if self.paths().is_empty() {
            self.runnable = false;

            eprintln!("No valid files or directories have been specified.");
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

    pub fn runnable(&self) -> bool {
        self.runnable
    }
}

#[cfg(test)]
mod tests {

    // #[test]
    // fn
}
