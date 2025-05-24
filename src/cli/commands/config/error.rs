use crate::services::{
    errors::EditorLauncherError, keyring::KeyringError, prompt::PromptServiceError,
};

#[derive(thiserror::Error, Debug)]
pub enum ConfigCommandError {
    #[error(transparent)]
    Keyring(#[from] KeyringError),

    #[error(transparent)]
    Prompt(#[from] PromptServiceError),

    #[error(transparent)]
    EditorLauncher(#[from] EditorLauncherError),
}
