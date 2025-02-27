use std::path::PathBuf;

use parse_settings::Config;

pub mod get_storage_dir;
pub mod parse_settings;

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

    pub fn initialize(debug: &bool) -> Result<AppContext, Box<dyn std::error::Error>> {
        let storage_dir = AppContext::get_storage_dir()?;
        let config = AppContext::parse_config(&storage_dir)?;

        Ok(AppContext::new(*debug, storage_dir, config))
    }
}
