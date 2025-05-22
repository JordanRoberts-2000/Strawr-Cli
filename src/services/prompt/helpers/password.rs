use inquire::{InquireError, Password, PasswordDisplayMode};

use crate::services::prompt::user::UserInputError;

pub fn password_prompt(
    display_mode: &PasswordDisplayMode,
    msg: &str,
) -> Result<String, UserInputError> {
    let prompt = Password::new(msg)
        .without_confirmation()
        .with_display_mode(*display_mode)
        .prompt();

    match prompt {
        Ok(input) => Ok(input),
        Err(InquireError::OperationInterrupted | InquireError::OperationCanceled) => {
            Err(UserInputError::Canceled)
        }
        Err(e) => Err(UserInputError::InquireError(e)),
    }
}
