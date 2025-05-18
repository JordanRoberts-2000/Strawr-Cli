use inquire::{Confirm, InquireError};

use crate::services::prompt::{
    traits::ConfirmPrompt,
    user::{UserInputError, UserInputRepo},
};

impl ConfirmPrompt for UserInputRepo {
    type Error = UserInputError;

    fn confirm(&self, msg: &str) -> Result<bool, UserInputError> {
        let prompt = Confirm::new(msg).with_default(false).prompt();

        match prompt {
            Ok(input) => Ok(input),
            Err(InquireError::OperationInterrupted | InquireError::OperationCanceled) => {
                Err(UserInputError::Canceled)
            }
            Err(e) => Err(UserInputError::InquireError(e)),
        }
    }
}
