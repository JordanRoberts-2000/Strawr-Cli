use crate::{
    constants::{KEYRING_ENCRYPTION_PASSWORD, KEYRING_SERVICE},
    error::Result,
    services::{
        crypto::{decrypt_data, ENCRYPTION_PREFIX},
        keychain::keychain,
    },
    utils::to_clipboard,
};

use super::GrabManager;

impl GrabManager {
    pub fn get_entry(&self, val: &String) -> Result<()> {
        let final_value: String = if val.starts_with(ENCRYPTION_PREFIX) {
            log::trace!("Value requires decryption");
            let password = keychain(KEYRING_SERVICE, KEYRING_ENCRYPTION_PASSWORD)?;
            decrypt_data(val, &password)?
        } else {
            val.clone()
        };

        to_clipboard(&final_value)?;

        println!("Value saved to clipboard");
        Ok(())
    }
}
