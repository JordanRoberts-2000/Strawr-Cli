use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Watcher,
    Img {
        #[arg(short, long)]
        path: Option<String>,
    },
}

// #[derive(Subcommand, Debug)]
// pub enum Commands {
//     Watcher,
//     Img {
//         #[command(subcommand)]
//         subcommands: ImgCommands,
//     },
// }

// #[derive(Subcommand, Debug)]
// pub enum ImgCommands {
//     Opt {
//         #[arg(short, long)]
//         path: Option<String>,
//     },
// }
