use clap::Parser;
use cli::{args::Commands, Cli};
use config::logging::initialize_logger;

pub mod cli;
mod commands;
pub mod config;
mod utils;

fn main() {
    dotenv::dotenv().ok();
    initialize_logger();
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
        Commands::Tw => println!(
            "Tailwind hexcode + copy to clipboard, maybe just a editable file where typing x = y"
        ),
        Commands::Playground => println!("Snippet"),
        Commands::Backup => println!("Snippet"),
        Commands::Uninstall => println!("Snippet"),
    }
}
