use self::utils::{logger, time_execution};

mod cli;
mod config;
mod constants;
mod context;
mod core;
mod error;
mod services;
mod traits;
mod utils;

pub(crate) use {cli::commands::template, core::*};
pub use {
    cli::{cli::Cli, commands},
    config::CliConfig,
    context::CliContext,
    error::CliError,
};

pub fn run_cli(cli: &Cli) -> Result<(), CliError> {
    logger::initialize(cli.debug);
    let ctx = CliContext::initialize(&cli.debug)?;

    time_execution(|| cli.command.execute(&ctx))?;

    Ok(())
}
