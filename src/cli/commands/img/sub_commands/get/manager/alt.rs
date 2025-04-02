use crate::{
    cli::commands::img::ImgError,
    constants::{KEYRING_OPEN_API_KEY, KEYRING_SERVICE},
    services::ai,
    utils,
};

use super::GetManager;

impl GetManager {
    pub fn handle_alt(&mut self) -> Result<(), ImgError> {
        let api_key = utils::keychain(KEYRING_SERVICE, KEYRING_OPEN_API_KEY)?;
        let data_url = self.img.max_size(400).webp()?.data_url()?;

        let description = ai::sync::alt_tag(&api_key, &data_url)?;
        utils::clipboard(&description)?;
        println!("Alt Text: {}", description);

        Ok(())
    }
}
