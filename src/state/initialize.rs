use crate::{
    config::Config,
    error::CliError,
    utils::{editor::CliEditor, input::UserInput},
};

use super::AppContext;

impl AppContext {
    pub fn initialize(debug: &bool) -> Result<Self, CliError> {
        let storage_dir = Self::get_storage_dir()?;
        let config = Config::parse(&storage_dir)?;
        let input = Box::new(UserInput);
        let editor = Box::new(CliEditor);

        log::trace!("App context initialized");
        Ok(Self {
            debug: *debug,
            storage_dir,
            config,
            input,
            editor,
        })
    }
}
