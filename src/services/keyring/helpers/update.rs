use {
    inquire::PasswordDisplayMode,
    keyring::{Entry, Error},
};

use crate::services::{
    keyring::{keyring as set_keyring, KeyringError},
    prompt::{confirm_prompt, password_prompt},
};

pub fn update_keyring(
    service: &str,
    field: &str,
    password_display: &PasswordDisplayMode,
) -> Result<(), KeyringError> {
    let keyring = Entry::new(service, field).map_err(KeyringError::Initialization)?;

    match keyring.get_password() {
        Ok(_) => {
            let msg = format!("Enter new value for '{}':", field);
            let input = password_prompt(password_display, &msg)?;

            keyring
                .set_password(&input)
                .map_err(KeyringError::SetKeyringPassword)?;
        }
        Err(err) => match err {
            Error::NoEntry => {
                let msg =
                    format!("No value currenly exists for {field}, would you like to set one?");
                if confirm_prompt(&msg)? {
                    set_keyring(service, field, password_display)?;
                }
            }
            _ => return Err(KeyringError::GetKeyringPassword(err)),
        },
    };

    Ok(())
}
