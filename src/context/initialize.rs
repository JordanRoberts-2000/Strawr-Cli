use crate::{services::storage, CliConfig, CliContext, CliError, CliService};

impl CliContext {
    pub fn initialize(debug: &bool) -> Result<Self, CliError> {
        let storage_dir = storage::initialize_storage_dir()?;
        let config = CliConfig::parse(&storage_dir)?;
        let service = CliService::new(&config);

        log::trace!("App context initialized");
        Ok(Self {
            debug: *debug,
            storage_dir,
            config,
            service,
        })
    }
}
