use super::user::UserInputError;

#[derive(thiserror::Error, Debug)]
pub enum PromptServiceError {
    #[error(transparent)]
    UserInput(#[from] UserInputError),

    #[error("invalid response from prompt: {0}")]
    InvalidResponse(String),

    #[error("no options provided to prompt")]
    EmptyOptions,

    #[error("default index {0} is out of range (0..{1})")]
    InvalidDefaultIndex(usize, usize),
}
