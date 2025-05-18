use crate::{
    prompt::{traits::CliInput, UserInput},
    services::storage,
    CliConfig, CliContext, CliError,
};

use super::CliService;

impl<P> CliContext<P>
where
    P: CliInput,
{
    pub fn initialize(debug: &bool) -> Result<Self, CliError> {
        let storage_dir = storage::initialize_storage_dir()?;
        let config = CliConfig::parse(&storage_dir)?;

        log::trace!("App context initialized");
        Ok(Self {
            debug: *debug,
            storage_dir,
            config,
            service: CliService<P> { prompt: P },
        })
    }
}
