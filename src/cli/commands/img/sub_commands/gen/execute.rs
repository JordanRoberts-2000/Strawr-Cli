use super::args::Gen;
use crate::{
    cli::commands::img::{sub_commands::gen::manager::GenManager, ImgError},
    services::{ai, img::Img},
    state::AppContext,
    utils::{self, Keychain},
};

impl Gen {
    pub fn execute(&self, ctx: &AppContext) -> Result<(), ImgError> {
        let mut manager = GenManager::new(ctx, &self);
        manager.handle_args();

        let api_key = utils::keychain(Keychain::OpenAiKey)?;

        let url = ai::sync::image(&api_key, &self.description).generate()?;

        Img::download(&url.to_string())?.save()?;

        Ok(())
    }
}
