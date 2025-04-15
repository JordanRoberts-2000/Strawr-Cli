use crate::{
    cli::commands::grab::{GrabError, GrabManager},
    services::crypto::{decrypt, ENCRYPTION_PREFIX},
    utils::{self, Keychain},
};

impl GrabManager {
    pub fn get_entry(&self, key: &String) -> Result<(), GrabError> {
        let val = match self.data_map.get(key) {
            Some(val) => val,
            None => return Err(GrabError::KeyNotFound(key.clone())),
        };

        let final_value = if val.starts_with(ENCRYPTION_PREFIX) {
            log::trace!("Value requires decryption");
            let password = utils::keychain(Keychain::Password)?;
            decrypt(val, &password)?
        } else {
            val.clone()
        };

        utils::clipboard(&final_value)?;
        println!("Value saved to clipboard");

        Ok(())
    }
}
