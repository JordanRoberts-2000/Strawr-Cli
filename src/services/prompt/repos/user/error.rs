#[derive(thiserror::Error, Debug)]
pub enum UserInputError {
    #[error("Canceled")]
    Canceled,
    #[error("inquire error: {0}")]
    InquireError(#[from] inquire::InquireError),
    #[error("invalid input: {0}")]
    InvalidInput(String),
}
