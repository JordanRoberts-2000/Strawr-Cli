use crate::{ai::AiError, services::prompt::PromptServiceError};

#[derive(thiserror::Error, Debug)]
pub enum SuggestCmdError {
    #[error(transparent)]
    Ai(#[from] AiError),

    #[error(transparent)]
    Prompt(#[from] PromptServiceError),
}
