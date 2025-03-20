use clap::Parser;
use cli::{
    commands::{grab::handler::Grab, img::handler::Img},
    Cli, Commands,
};
use error::Result;
use state::AppContext;
use utils::logger;

pub mod cli;
pub mod config;
pub mod constants;
pub mod error;
pub mod services;
pub mod state;
pub mod utils;

pub fn run_cli() -> Result<()> {
    let cli = Cli::parse();

    logger::initialize(cli.debug);
    let ctx = AppContext::initialize(&cli.debug)?;

    match cli.commands {
        Commands::Grab(args) => Grab::execute(&args, &ctx),
        Commands::Img(args) => Img::execute(&args, &ctx),
        // Commands::Temp ??? - creates random folder in ./temp
        // Commands::Playground(ref playground) => playground.handle_command(&ctx),
        // Commands::Template => println!("Template"),

        // variable options,weights, italics, axes, all available with terminal providing options after choosing font
        // strawr font "inter" --output "src/fonts" --css "src/styles/fonts.css" // if not exist create, if exists append
        // Commands::Font => println!("Font"),
        // Commands::Add => println!("Add"),
        // Commands::Snippets => println!("Snippet"),

        // Commands::Backup => println!("backs up .cli, --destination --zip"), // encrypt stored data
        // Commands::Uninstall(ref uninstall) => uninstall.handle_command(&ctx), // delete keyring
        // change to config, allow reset or removal of cheychain password, openai api key
        // Commands::Open(ref open) => open.handle_command(&ctx),
        _ => Ok(()),
    }?;

    Ok(())
}
