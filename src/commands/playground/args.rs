use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct PlaygroundCommand {
    #[arg(short, long)]
    pub replace: bool,

    #[command(subcommand)]
    pub command: Option<PlaygroundSubcommands>,
}

#[derive(Subcommand, Debug)]
pub enum PlaygroundSubcommands {
    Save,
    Reset,
}
