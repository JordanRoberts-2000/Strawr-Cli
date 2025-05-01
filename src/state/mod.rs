use std::path::PathBuf;

use crate::{
    config::Config,
    error::CliError,
    utils::input::{CliInput, UserInput},
};

pub mod error;
pub mod storage_dir;

pub struct AppContext {
    pub debug: bool,
    pub storage_dir: PathBuf,
    pub config: Config,
    pub input: Box<dyn CliInput>,
}

impl AppContext {
    pub fn new(
        debug: bool,
        storage_dir: PathBuf,
        config: Config,
        input: Box<dyn CliInput>,
    ) -> Self {
        Self {
            debug,
            storage_dir,
            config,
            input,
        }
    }

    pub fn initialize(debug: &bool) -> Result<Self, CliError> {
        let storage_dir = Self::get_storage_dir()?;
        let config = Config::parse(&storage_dir)?;
        let input = Box::new(UserInput);

        log::trace!("App context initialized");
        Ok(Self {
            debug: *debug,
            storage_dir,
            config,
            input,
        })
    }
}
