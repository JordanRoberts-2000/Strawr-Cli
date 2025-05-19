use super::user::UserInputError;

#[derive(thiserror::Error, Debug)]
pub enum PromptServiceError {
    #[error(transparent)]
    UserInput(#[from] UserInputError),

    #[error("invalid response from prompt: {0}")]
    InvalidResponse(String),
}
