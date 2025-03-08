use clap::Parser;
use cli::Cli;
use commands::Commands;
use error::Result;
use state::AppContext;
use utils::logger::init_logger;

pub mod cli;
pub mod commands;
pub mod config;
pub mod crypto;
pub mod error;
pub mod state;
pub mod utils;

pub fn run_app() -> Result<()> {
    let cli = Cli::parse();

    init_logger(cli.debug);
    let ctx = AppContext::initialize(&cli.debug)?;

    match cli.commands {
        Commands::Img(ref img) => img.handle_command(&ctx),
        // Commands::Add => println!("Add"),
        // Commands::Template => println!("Template"),

        // Commands::Temp ??? - creates random folder in ./temp

        // variable options,weights, italics, axes, all available with terminal providing options after choosing font
        // strawr font "inter" --output "src/fonts" --css "src/styles/fonts.css" // if not exist create, if exists append
        // Commands::Font => println!("Font"),

        // Commands::Snippets => println!("Snippet"),
        Commands::Grab(ref grab) => grab.handle_command(&ctx),
        _ => Ok(()),
        // Commands::Playground(ref playground) => playground.handle_command(&ctx),

        // Commands::Backup => println!("backs up .cli, --destination --zip"), // encrypt stored data
        // Commands::Uninstall(ref uninstall) => uninstall.handle_command(&ctx), // delete keyring
        // change to config, allow reset or removal of cheychain password, openai api key
        // Commands::Open(ref open) => open.handle_command(&ctx),
    }?;

    Ok(())
}
