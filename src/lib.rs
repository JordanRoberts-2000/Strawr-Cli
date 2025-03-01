use clap::Parser;
use cli::Cli;
use commands::Commands;
use error::Result;
use state::AppContext;
use utils::logger::init_logger;

pub mod cli;
pub mod commands;
pub mod config;
pub mod error;
pub mod state;
pub mod utils;

pub fn run_app() -> Result<()> {
    let cli = Cli::parse();

    init_logger(cli.debug);
    let ctx = AppContext::initialize(&cli.debug)?;

    match cli.commands {
        // Commands::Img(ref img) => img.handle_command(),
        // Commands::Add => println!("Add"),
        // Commands::Template => println!("Template"),
        // strawr font "inter" --variable -weight "100-900" -italic "400-500" --output "googleFonts"
        // Commands::Font => println!("Font"),
        // Commands::Snippets => println!("Snippet"),
        Commands::Grab(ref grab) => grab.handle_command(&ctx),
        _ => Ok(()),
        // Commands::Playground(ref playground) => playground.handle_command(&ctx),

        // Commands::Backup => println!("backs up .cli, --destination --zip"),
        // Commands::Uninstall(ref uninstall) => uninstall.handle_command(&ctx),
        // Commands::Open(ref open) => open.handle_command(&ctx),
    }?;

    Ok(())
}
