use crate::{
    config::constants::{ENCRYPTION_PREFIX, KEYRING_ENCRYPTION_PASSWORD, KEYRING_SERVICE},
    crypto::{decrypt_data, get_or_prompt_keyring},
    error::Result,
    utils::add_to_clipboard,
};

use super::GrabService;

impl GrabService {
    pub fn get_entry(&self, val: &String) -> Result<()> {
        log::debug!("Processing value for clipboard: '{}'", val);

        let final_value: String = if val.starts_with(ENCRYPTION_PREFIX) {
            let password = get_or_prompt_keyring(KEYRING_SERVICE, KEYRING_ENCRYPTION_PASSWORD)?;
            decrypt_data(val, &password)?
        } else {
            val.clone()
        };

        add_to_clipboard(&final_value)?;
        Ok(())
    }
}
