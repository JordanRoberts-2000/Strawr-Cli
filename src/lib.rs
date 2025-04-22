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
        Commands::Temp(cmd) => cmd.execute(&ctx)?,
        Commands::Template(cmd) => cmd.execute(&ctx)?,

        Commands::Backup => todo!(),
        Commands::Config => todo!(),
        Commands::Uninstall => todo!(),
    };

    let duration = start_time.elapsed();
    log::info!("⏱️ Completed in {:.2?}", duration);

    Ok(())
}
