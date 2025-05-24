use crate::ai::AiError;

#[derive(thiserror::Error, Debug)]
pub enum AiServiceError {
    #[error(transparent)]
    AiError(#[from] AiError),
}
