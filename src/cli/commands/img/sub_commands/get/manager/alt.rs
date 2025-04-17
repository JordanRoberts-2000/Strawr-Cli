use crate::{
    cli::commands::img::ImgError,
    services::ai,
    utils::{self, Keyring},
};

use super::GetManager;

impl GetManager {
    pub fn handle_alt(&mut self) -> Result<(), ImgError> {
        let api_key = Keyring::OpenAiKey.retrieve()?;
        let data_url = self.img.max_size(400).webp()?.data_url()?;

        let description = ai::sync::alt_tag(&api_key, &data_url)?;
        utils::clipboard(&description)?;
        println!("Alt Text: {}", description);

        Ok(())
    }
}
