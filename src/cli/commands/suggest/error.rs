use crate::{
    ai::AiError,
    services::{ai::AiServiceError, keyring::KeyringError, prompt::PromptServiceError},
};

#[derive(thiserror::Error, Debug)]
pub enum SuggestCmdError {
    #[error(transparent)]
    Ai(#[from] AiError),

    #[error(transparent)]
    AiService(#[from] AiServiceError),

    #[error(transparent)]
    Keyring(#[from] KeyringError),

    #[error(transparent)]
    Prompt(#[from] PromptServiceError),

    #[error("Unable to generate 8 valid names")]
    GenerationUnsuccessful,
}
