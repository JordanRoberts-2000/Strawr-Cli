use crate::error::{Error, Result};
use keyring::Entry;
use rpassword::read_password;

pub fn get_or_prompt_keyring(service: &str, field: &str) -> Result<String> {
    log::trace!("Attempting to retrieve keyring value for {}", field);

    let keyring = Entry::new(service, field)
        .map_err(|e| Error::Keyring(e, format!("failed to initialize '{}'", field)))?;

    match keyring.get_password() {
        Ok(password) => {
            log::debug!("keyring value for '{}' retrieved successfully", field);
            Ok(password)
        }
        Err(err) => match err {
            keyring::Error::NoEntry => {
                println!(
                    "No value detected for '{}'. Please enter one:",
                    field.replace("_", " ")
                );
                let input = read_password()
                    .map_err(|e| Error::Custom(format!("failed to read password, {}", e)))?;
                keyring.set_password(&input).map_err(|e| {
                    Error::Keyring(e, format!("failed to set password for field '{}'", field))
                })?;

                log::debug!("keyring '{}' updated successfully", field);
                Ok(input)
            }
            _ => Err(Error::Keyring(
                err,
                format!("failed to retrieve '{}'", field),
            )),
        },
    }
}
