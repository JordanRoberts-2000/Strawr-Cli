use clap::Parser;
use cli::{args::Commands, Cli};

pub mod cli;
mod commands;
mod utils;

fn main() {
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
        Commands::Playground => println!("Snippet"),
        Commands::Backup => println!("Snippet"),
        Commands::Uninstall => println!("Snippet"),
    }
}
