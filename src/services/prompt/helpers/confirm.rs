use inquire::{Confirm, InquireError};

use crate::services::prompt::user::UserInputError;

pub fn confirm_prompt(msg: &str) -> Result<bool, UserInputError> {
    let prompt = Confirm::new(msg).with_default(false).prompt();

    match prompt {
        Ok(input) => Ok(input),
        Err(InquireError::OperationInterrupted | InquireError::OperationCanceled) => {
            Err(UserInputError::Canceled)
        }
        Err(e) => Err(UserInputError::InquireError(e)),
    }
}
