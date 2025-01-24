use clap::Parser;
use commands::Commands;

pub mod commands;
pub mod config;
mod utils;

#[derive(Parser, Debug)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,
}

fn main() {
    let cli = Cli::parse();

    match cli.commands {
        // Commands::Img(ref img) => img.handle_command(),
        Commands::Add => println!("Add"),
        Commands::Template => println!("Template"),
        // strawr font "inter" --variable -weight "100-900" -italic "400-500" --output "googleFonts"
        Commands::Font => println!("Font"),
        Commands::Project => println!("Project"),
        Commands::Snippets => println!("Snippet"),
        Commands::Grab => println!("Editable file where typing x = y"),
        Commands::Playground(ref playground) => playground.handle_command(),
        Commands::Backup => println!("Snippet"),
        Commands::Uninstall => println!("Snippet"),
        Commands::Config => println!("load config from backup file, open, reset"),
    }
}
