use parse_config::Config;
use std::path::PathBuf;

use crate::error::Result;

pub mod get_storage_dir;
pub mod parse_config;

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

    pub fn initialize(debug: &bool) -> Result<AppContext> {
        let storage_dir = AppContext::get_storage_dir()?;
        let config = AppContext::parse_config(&storage_dir)?;

        Ok(AppContext::new(*debug, storage_dir, config))
    }
}
