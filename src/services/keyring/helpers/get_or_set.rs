use {
    inquire::PasswordDisplayMode,
    keyring::{Entry, Error},
};

use crate::services::{keyring::KeyringError, prompt::password_prompt};

pub fn keyring(
    service: &str,
    field: &str,
    password_display: &PasswordDisplayMode,
) -> Result<String, KeyringError> {
    let keyring = Entry::new(service, field).map_err(KeyringError::Initialization)?;

    match keyring.get_password() {
        Ok(password) => Ok(password),
        Err(err) => match err {
            Error::NoEntry => {
                let msg = format!("Please enter value for '{}':", field);
                let input = password_prompt(password_display, &msg)?;
                keyring
                    .set_password(&input)
                    .map_err(KeyringError::SetKeyringPassword)?;
                Ok(input)
            }
            _ => Err(KeyringError::GetKeyringPassword(err)),
        },
    }
}
