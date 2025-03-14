use clap::Parser;

pub mod commands;
pub use commands::Commands;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,

    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub debug: bool,
}
