use crate::{
    constants::{KEYRING_OPEN_API_KEY, KEYRING_SERVICE},
    error::Result,
    services::{ai, keychain::keychain},
    utils::to_clipboard,
};

use super::GetManager;

impl GetManager {
    pub fn handle_alt(&mut self) -> Result<()> {
        let api_key = keychain(KEYRING_SERVICE, KEYRING_OPEN_API_KEY)?;
        let data_url = self.img.max_size(400).webp()?.data_url()?;

        let description = ai::sync::alt_tag(&api_key, &data_url)?;
        to_clipboard(&description)?;
        println!("Alt Text: {}", description);

        Ok(())
    }
}
