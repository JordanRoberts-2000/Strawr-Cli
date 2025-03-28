use crate::{
    cli::commands::img::ImgError,
    constants::{KEYRING_OPEN_API_KEY, KEYRING_SERVICE},
    error::Result,
    services::{ai, keychain::keychain},
    utils::to_clipboard,
};

use super::GetManager;

impl GetManager {
    pub fn handle_alt(&mut self) -> Result<()> {
        let api_key = keychain(KEYRING_SERVICE, KEYRING_OPEN_API_KEY)?;
        let data_url = self
            .img
            .max_size(400)
            .webp()
            .map_err(ImgError::ImgFailed)?
            .data_url()
            .map_err(ImgError::ImgFailed)?;

        let description = ai::sync::alt_tag(&api_key, &data_url).map_err(ImgError::AltTag)?;
        to_clipboard(&description)?;
        println!("Alt Text: {}", description);

        Ok(())
    }
}
