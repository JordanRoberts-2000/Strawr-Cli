use std::error::Error as StdError;

#[derive(thiserror::Error, Debug)]
pub enum PromptServiceError {
    #[error("Failed to prompt for search: {0}")]
    RepoSearch(#[source] Box<dyn StdError + Send + Sync>),

    #[error("Failed to prompt for select: {0}")]
    RepoSelect(#[source] Box<dyn StdError + Send + Sync>),

    #[error("Failed to prompt for text: {0}")]
    RepoText(#[source] Box<dyn StdError + Send + Sync>),

    #[error("Failed to prompt for confirm: {0}")]
    RepoConfirm(#[source] Box<dyn StdError + Send + Sync>),

    #[error("invalid response from prompt: {0}")]
    InvalidResponse(String),
}
