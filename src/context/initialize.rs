use crate::{
    services::storage::initialize_storage_dir,
    utils::{editor::CliEditor, input::UserInput},
    CliConfig, CliContext, CliError,
};

use super::core::CliService;

impl CliContext {
    pub fn initialize(debug: &bool) -> Result<Self, CliError> {
        let storage_dir = initialize_storage_dir()?;
        let config = CliConfig::parse(&storage_dir)?;

        let service = CliService {
            prompt: Box::new(UserInput),
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
