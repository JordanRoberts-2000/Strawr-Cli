use cli::Cli;
use error::CliError;
use state::AppContext;
use utils::{logger, time_execution};

pub mod cli;
pub mod config;
pub mod constants;
pub mod error;
pub mod services;
pub mod state;
pub mod utils;

pub fn run_cli(cli: &Cli) -> Result<(), CliError> {
    logger::initialize(cli.debug);
    let ctx = AppContext::initialize(&cli.debug)?;

    time_execution(|| cli.command.execute(&ctx))?;

    Ok(())
}
