use std::fmt;

use keyring::Entry;
use rpassword::read_password;

const KEYRING_SERVICE: &str = "strawrCli";

pub enum Keychain {
    Password,
    OpenAiKey,
}

impl fmt::Display for Keychain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Keychain::Password => "encryption password",
            Keychain::OpenAiKey => "open-ai api-key",
        };
        write!(f, "{}", value)
    }
}

pub fn keychain(field: Keychain) -> Result<String, KeyChainError> {
    log::trace!("Attempting to retrieve keyring value for {}", field);

    let field_str = field.to_string();
    let keyring = Entry::new(KEYRING_SERVICE, &field_str).map_err(KeyChainError::Initialization)?;

    match keyring.get_password() {
        Ok(password) => {
            log::debug!("keyring value for '{}' retrieved successfully", field_str);
            Ok(password)
        }
        Err(err) => match err {
            keyring::Error::NoEntry => {
                println!("Please enter value for '{}':", field_str);
                let input = read_password().map_err(KeyChainError::UserInputRead)?;
                keyring
                    .set_password(&input)
                    .map_err(KeyChainError::SetKeyringPassword)?;

                log::debug!("keyring '{}' updated successfully", field_str);
                Ok(input)
            }
            _ => Err(KeyChainError::GetKeyringPassword(err)),
        },
    }
}

#[derive(thiserror::Error, Debug)]
pub enum KeyChainError {
    #[error("failed to initialize keyring: {0}")]
    Initialization(keyring::Error),
    #[error("failed to read password: {0}")]
    UserInputRead(std::io::Error),
    #[error("failed to read password: {0}")]
    SetKeyringPassword(keyring::Error),
    #[error("failed to read password: {0}")]
    GetKeyringPassword(keyring::Error),
}
