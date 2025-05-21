use crate::services::prompt::user::UserInputError;

#[derive(thiserror::Error, Debug)]
pub enum KeyringError {
    #[error("Failed initializing keyring entry: {0}")]
    Initialization(#[source] keyring::Error),

    #[error("Failed to get keyring password: {0}")]
    GetKeyringPassword(#[source] keyring::Error),

    #[error("Failed to set keyring password: {0}")]
    SetKeyringPassword(#[source] keyring::Error),

    #[error("Failed deleting keyring entry: {0}")]
    DeleteKeyringPassword(#[source] keyring::Error),

    #[error("No keyring entry for service `{0}`, field `{1}`")]
    EntryNotFound(String, String),

    #[error(transparent)]
    UserInput(#[from] UserInputError),
}
