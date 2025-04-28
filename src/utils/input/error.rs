use inquire::InquireError;

#[derive(thiserror::Error, Debug)]
pub enum InputError {
    #[error("Canceled")]
    Canceled,
    #[error("inquire error: {0}")]
    PromptError(#[from] InquireError),
    #[error("invalid input: {0}")]
    InvalidInput(String),
}
