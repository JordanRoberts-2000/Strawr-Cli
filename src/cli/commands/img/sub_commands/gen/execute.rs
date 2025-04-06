use super::args::Gen;
use crate::{
    cli::commands::img::{sub_commands::gen::manager::GenManager, ImgError},
    constants::{KEYRING_OPEN_API_KEY, KEYRING_SERVICE},
    services::{ai, img::Img},
    state::AppContext,
    utils,
};

impl Gen {
    pub fn execute(&self, ctx: &AppContext) -> Result<(), ImgError> {
        let mut manager = GenManager::new(ctx, &self);
        manager.handle_args();

        let api_key = utils::keychain(KEYRING_SERVICE, KEYRING_OPEN_API_KEY)?;

        let url = ai::sync::image(&api_key, &self.description).generate()?;

        Img::download(&url.to_string())?.save()?;

        Ok(())
    }
}
