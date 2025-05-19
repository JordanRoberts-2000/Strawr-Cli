use inquire::{InquireError, Password, PasswordDisplayMode};

use crate::services::prompt::{
    traits::PasswordPrompt,
    user::{UserInputError, UserInputRepo},
};

impl PasswordPrompt for UserInputRepo {
    fn password(
        &self,
        display_mode: &PasswordDisplayMode,
        msg: &str,
    ) -> Result<String, UserInputError> {
        let prompt = Password::new(msg).with_display_mode(*display_mode).prompt();

        match prompt {
            Ok(input) => Ok(input),
            Err(InquireError::OperationInterrupted | InquireError::OperationCanceled) => {
                Err(UserInputError::Canceled)
            }
            Err(e) => Err(UserInputError::InquireError(e)),
        }
    }
}
