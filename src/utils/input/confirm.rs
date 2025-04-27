use inquire::{Confirm, InquireError};

use super::{InputError, UserInput};

pub trait ConfirmInput {
    fn confirm(&self, msg: &str) -> Result<bool, InputError>;
}

impl ConfirmInput for UserInput {
    fn confirm(&self, msg: &str) -> Result<bool, InputError> {
        let prompt = Confirm::new(msg).with_default(false).prompt();

        match prompt {
            Ok(input) => Ok(input),
            Err(InquireError::OperationInterrupted | InquireError::OperationCanceled) => {
                Err(InputError::Canceled)
            }
            Err(e) => Err(InputError::PromptError(e)),
        }
    }
}
