use crate::{
    utils::{editor::CliEditor, input::UserInput},
    CliConfig, CliContext, CliError,
};

use super::{core::CliService, helpers::get_storage_dir};

impl CliContext {
    pub fn initialize(debug: &bool) -> Result<Self, CliError> {
        let storage_dir = get_storage_dir()?;
        let config = CliConfig::parse(&storage_dir)?;

        let service = CliService {
            input: Box::new(UserInput),
            editor_launcher: Box::new(CliEditor),
        };

        log::trace!("App context initialized");
        Ok(Self {
            debug: *debug,
            storage_dir,
            config,
            service,
        })
    }
}
