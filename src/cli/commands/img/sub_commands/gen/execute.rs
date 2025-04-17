use super::args::Gen;
use crate::{
    cli::commands::img::{sub_commands::gen::manager::GenManager, ImgError},
    services::{ai, img::Img},
    state::AppContext,
    utils::Keyring,
};

impl Gen {
    pub fn execute(&self, ctx: &AppContext) -> Result<(), ImgError> {
        let mut manager = GenManager::new(ctx, &self);
        manager.handle_args();

        let api_key = Keyring::OpenAiKey.retrieve()?;
        let url = ai::sync::image(&api_key, &self.description).generate()?;

        Img::download(&url.to_string())?.save()?;
        Ok(())
    }
}
