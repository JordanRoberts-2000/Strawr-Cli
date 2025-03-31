use keyring::Entry;
use rpassword::read_password;

pub fn keychain(service: &str, field: &str) -> Result<String, KeyChainError> {
    log::trace!("Attempting to retrieve keyring value for {}", field);

    let keyring = Entry::new(service, field).map_err(KeyChainError::Initialization)?;

    match keyring.get_password() {
        Ok(password) => {
            log::debug!("keyring value for '{}' retrieved successfully", field);
            Ok(password)
        }
        Err(err) => match err {
            keyring::Error::NoEntry => {
                println!("Please enter value for '{}':", field);
                let input = read_password().map_err(KeyChainError::UserInputRead)?;
                keyring
                    .set_password(&input)
                    .map_err(KeyChainError::SetKeyringPassword)?;

                log::debug!("keyring '{}' updated successfully", field);
                Ok(input)
            }
            _ => Err(KeyChainError::GetKeyringPassword(err)),
        },
    }
}

#[derive(thiserror::Error, Debug)]
pub enum KeyChainError {
    #[error("failed to initialize keyring: {0}")]
    Initialization(#[source] keyring::Error),
    #[error("failed to read password: {0}")]
    UserInputRead(#[source] std::io::Error),
    #[error("failed to read password: {0}")]
    SetKeyringPassword(#[source] keyring::Error),
    #[error("failed to read password: {0}")]
    GetKeyringPassword(#[source] keyring::Error),
}
