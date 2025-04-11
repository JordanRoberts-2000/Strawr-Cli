use std::time::Instant;

use cli::{Cli, Commands};
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

pub fn run_cli(cli: Cli) -> Result<()> {
    logger::initialize(cli.debug);
    let ctx = AppContext::initialize(&cli.debug)?;
    let start_time = Instant::now();

    match cli.commands {
        Commands::Grab(cmd) => cmd.execute(&ctx)?,
        Commands::Img(cmd) => cmd.execute(&ctx)?,
        // Commands::Temp => todo!(),
        // Commands::Playground => todo!(),
        Commands::Template => todo!(),
        Commands::Font => todo!(),
        Commands::Backup => todo!(),
    };

    let duration = start_time.elapsed();
    log::info!("⏱️ Completed in {:.2?}", duration);

    Ok(())
}
