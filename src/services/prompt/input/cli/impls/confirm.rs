use inquire::{Confirm, InquireError};

use crate::services::prompt::{traits::ConfirmPrompt, PromptError, UserInput};

impl ConfirmPrompt for UserInput {
    fn confirm(&self, msg: &str) -> Result<bool, PromptError> {
        let prompt = Confirm::new(msg).with_default(false).prompt();

        match prompt {
            Ok(input) => Ok(input),
            Err(InquireError::OperationInterrupted | InquireError::OperationCanceled) => {
                Err(PromptError::Canceled)
            }
            Err(e) => Err(PromptError::InquireError(e)),
        }
    }
}
