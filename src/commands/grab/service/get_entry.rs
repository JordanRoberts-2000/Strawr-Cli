use crate::{
    config::constants::{ENCRYPTION_PREFIX, KEYRING_ENCRYPTION_PASSWORD, KEYRING_SERVICE},
    crypto::{decrypt_data, get_or_prompt_keyring},
    error::Result,
    utils::add_to_clipboard,
};

use super::GrabService;

impl GrabService {
    pub fn get_entry(&self, val: &String) -> Result<()> {
        let final_value: String = if val.starts_with(ENCRYPTION_PREFIX) {
            log::trace!("Value requires decryption");
            let password = get_or_prompt_keyring(KEYRING_SERVICE, KEYRING_ENCRYPTION_PASSWORD)?;
            decrypt_data(val, &password)?
        } else {
            val.clone()
        };

        add_to_clipboard(&final_value)?;

        println!("Value saved to clipboard");
        Ok(())
    }
}
