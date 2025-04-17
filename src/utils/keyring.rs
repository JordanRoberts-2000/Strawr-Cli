use std::fmt;

use keyring::{Entry, Error};
use rpassword::read_password;

const KEYRING_SERVICE: &str = "strawrCli";

pub enum Keyring {
    Password,
    OpenAiKey,
}

impl fmt::Display for Keyring {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Keyring::Password => "encryption password",
            Keyring::OpenAiKey => "open-ai api-key",
        };
        write!(f, "{}", value)
    }
}

impl Keyring {
    pub fn retrieve(&self) -> Result<String, KeyringError> {
        keyring(KEYRING_SERVICE, &self.to_string())
    }
}

pub fn keyring(service: &str, field: &str) -> Result<String, KeyringError> {
    log::trace!("Attempting to retrieve keyring value for {}", field);

    let keyring = Entry::new(service, field).map_err(KeyringError::Initialization)?;

    match keyring.get_password() {
        Ok(password) => {
            log::debug!("keyring value for '{}' retrieved successfully", field);
            Ok(password)
        }
        Err(err) => match err {
            Error::NoEntry => {
                println!("Please enter value for '{}':", field);
                let input = read_password().map_err(KeyringError::UserInputRead)?;
                keyring
                    .set_password(&input)
                    .map_err(KeyringError::SetKeyringPassword)?;

                log::debug!("keyring '{}' updated successfully", field);
                Ok(input)
            }
            _ => Err(KeyringError::GetKeyringPassword(err)),
        },
    }
}

#[derive(thiserror::Error, Debug)]
pub enum KeyringError {
    #[error("failed to initialize keyring: {0}")]
    Initialization(keyring::Error),
    #[error("failed to read password: {0}")]
    UserInputRead(std::io::Error),
    #[error("failed to read password: {0}")]
    SetKeyringPassword(keyring::Error),
    #[error("failed to read password: {0}")]
    GetKeyringPassword(keyring::Error),
}
