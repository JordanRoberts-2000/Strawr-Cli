use inquire::{InquireError, Text};

use crate::services::prompt::{
    traits::TextPrompt,
    user::{UserInputError, UserInputRepo},
};

impl TextPrompt for UserInputRepo {
    fn text(&self, msg: &str) -> Result<String, UserInputError> {
        match Text::new(msg).prompt() {
            Ok(input) => Ok(input),
            Err(InquireError::OperationInterrupted | InquireError::OperationCanceled) => {
                Err(UserInputError::Canceled)
            }
            Err(e) => Err(UserInputError::InquireError(e)),
        }
    }
}
