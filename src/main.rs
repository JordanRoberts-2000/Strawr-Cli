use args::{Cli, Commands};
use clap::Parser;
use commands::img::handle_img;
use dotenv::from_path;

mod args;
mod commands;
mod utils;

fn main() {
    load_env();
    let cli = Cli::parse();

    match cli.commands {
        Commands::Img { path } => handle_img(path),
        Commands::Watcher => println!("Watching"),
    }
}

fn load_env() {
    if let Err(err) = from_path("/Users/jordanroberts/Documents/dev/Projects/main/rustCli/.env") {
        panic!("Error loading .env file: {}", err);
    }
}
