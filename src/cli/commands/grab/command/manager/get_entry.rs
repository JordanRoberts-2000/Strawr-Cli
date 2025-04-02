use crate::{
    cli::commands::grab::GrabError,
    constants::{KEYRING_ENCRYPTION_PASSWORD, KEYRING_SERVICE},
    services::crypto::{decrypt, ENCRYPTION_PREFIX},
    utils,
};

use super::GrabManager;

impl GrabManager {
    pub fn get_entry(&self, val: &String) -> Result<(), GrabError> {
        let final_value: String = if val.starts_with(ENCRYPTION_PREFIX) {
            log::trace!("Value requires decryption");
            let password = utils::keychain(KEYRING_SERVICE, KEYRING_ENCRYPTION_PASSWORD)?;
            decrypt(val, &password)?
        } else {
            val.clone()
        };

        utils::clipboard(&final_value)?;

        println!("Value saved to clipboard");
        Ok(())
    }
}
