use keyring::Entry;
use rpassword::read_password;

use crate::services::keychain::error::{KeychainError, Result};

pub mod error;

pub fn get_or_prompt_keyring(service: &str, field: &str) -> Result<String> {
    log::trace!("Attempting to retrieve keyring value for {}", field);

    let keyring = Entry::new(service, field).map_err(|e| KeychainError::Keyring {
        source: e,
        context: format!("failed to initialize keyring"),
    })?;

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
                let input = read_password().map_err(KeychainError::ReadPasswordFailed)?;
                keyring
                    .set_password(&input)
                    .map_err(|e| KeychainError::Keyring {
                        source: e,
                        context: format!("failed to set password for field '{}'", field),
                    })?;

                log::debug!("keyring '{}' updated successfully", field);
                Ok(input)
            }
            _ => Err(KeychainError::Keyring {
                source: err,
                context: format!("failed to retrieve '{}'", field),
            }),
        },
    }
}
