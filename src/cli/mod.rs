use clap::Parser;
use commands::Command;

pub mod commands;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,

    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub debug: bool,
}
