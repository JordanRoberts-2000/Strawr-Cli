use args::Commands;
use clap::Parser;

pub mod args;
pub mod img;

#[derive(Parser, Debug)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,
}
