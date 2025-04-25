use std::path::PathBuf;

use crate::{config::Config, error::CliError};

pub mod error;
pub mod get_storage_dir;

#[derive(Debug)]
pub struct AppContext {
    pub debug: bool,
    pub storage_dir: PathBuf,
    pub config: Config,
}

impl AppContext {
    pub fn new(debug: bool, storage_dir: PathBuf, config: Config) -> Self {
        Self {
            debug,
            storage_dir,
            config,
        }
    }

    pub fn initialize(debug: &bool) -> Result<AppContext, CliError> {
        let storage_dir = AppContext::get_storage_dir()?;
        let config = Config::parse(&storage_dir)?;

        log::trace!("App context initialized");
        Ok(AppContext::new(*debug, storage_dir, config))
    }
}
