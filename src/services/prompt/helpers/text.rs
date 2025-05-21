use inquire::{InquireError, Text};

use crate::services::prompt::user::UserInputError;

pub fn text_prompt(msg: &str) -> Result<String, UserInputError> {
    match Text::new(msg).prompt() {
        Ok(input) => Ok(input),
        Err(InquireError::OperationInterrupted | InquireError::OperationCanceled) => {
            Err(UserInputError::Canceled)
        }
        Err(e) => Err(UserInputError::InquireError(e)),
    }
}
