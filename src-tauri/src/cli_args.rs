use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Clone, Parser)]
#[command(name = "pmon", about = "GitHub Workflow Monitor")]
pub struct CliArgs {
    #[arg(
        short = 'e',
        long = "edit-config",
        help = "Open $EDITOR with the config file"
    )]
    pub(crate) edit_config: bool,
    #[arg(
        short = 'c',
        long = "config-file",
        value_name = "FILE",
        help = "Use an alternative config file"
    )]
    pub(crate) config_file: Option<PathBuf>,
}

pub fn load_cli_args() -> CliArgs {
    CliArgs::parse()
}
