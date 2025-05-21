use keyring::{Entry, Error};

use crate::services::keyring::KeyringError;

pub fn delete_keyring(service: &str, field: &str) -> Result<(), KeyringError> {
    let entry = Entry::new(service, field).map_err(KeyringError::Initialization)?;

    match entry.delete_credential() {
        Ok(()) => {
            println!("Deleted '{field}' successfully");
            Ok(())
        }
        Err(Error::NoEntry) => Err(KeyringError::EntryNotFound(
            service.to_string(),
            field.to_string(),
        )),
        Err(e) => Err(KeyringError::DeleteKeyringPassword(e)),
    }
}
