pub mod stodo_args;
pub mod clap_args;

use {
    stodo_args::CliConfig,
    clap_args::clap_args,
};

pub fn run_config() -> CliConfig {
    let clap_args = clap_args();
    CliConfig::from(&clap_args)
}
