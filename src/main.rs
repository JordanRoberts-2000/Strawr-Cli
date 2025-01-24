use clap::Parser;
use cli::Cli;
use commands::Commands;
use state::AppContext;

pub mod cli;
pub mod commands;
pub mod config;
pub mod state;
pub mod utils;

fn main() {
    let cli = Cli::parse();

    let config_path = cli.get_config_path();
    let ctx = AppContext::new(cli.debug, config_path);

    match cli.commands {
        // Commands::Img(ref img) => img.handle_command(),
        Commands::Add => println!("Add"),
        Commands::Template => println!("Template"),
        // strawr font "inter" --variable -weight "100-900" -italic "400-500" --output "googleFonts"
        Commands::Font => println!("Font"),
        Commands::Snippets => println!("Snippet"),
        Commands::Grab => println!("Editable file where typing x = y"),
        Commands::Playground(ref playground) => playground.handle_command(&ctx),

        Commands::Backup => println!("backs up .cli, --destination --zip"),
        Commands::Uninstall(ref uninstall) => uninstall.handle_command(&ctx),
        Commands::Open(ref open) => open.handle_command(&ctx),
    }
}
