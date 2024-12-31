use clap::Parser;
use cli::{args::Commands, Cli};
use dotenv::from_path;

pub mod cli;
mod commands;
mod utils;

fn main() {
    load_env();
    let cli = Cli::parse();

    match cli.commands {
        Commands::Img(ref img) => img.handle_command(),
        Commands::Watcher => println!("Watching"),
        Commands::Add => println!("Add"),
        Commands::Template => println!("Template"),
        Commands::Font => println!("Font"),
        Commands::Project => println!("Project"),
        Commands::Ping => println!("Ping"),
        Commands::Snippet => println!("Snippet"),
        Commands::Tw => println!("Tailwind hexcode + copy to clipboard"),
    }
}

fn load_env() {
    if let Err(err) = from_path("/Users/jordanroberts/Documents/dev/Projects/main/rustCli/.env") {
        panic!("Error loading .env file: {}", err);
    }
}
