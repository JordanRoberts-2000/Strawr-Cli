use clap::Parser;

use crate::commands::Commands;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,

    #[arg(
        short,
        long,
        value_name = "FOLDERPATH",
        help = "Path to store configuration files"
    )]
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub debug: bool,
}
