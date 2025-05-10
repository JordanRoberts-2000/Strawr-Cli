use inquire::{InquireError, Text};

use crate::services::prompt::{traits::TextPrompt, PromptError, UserInput};

impl TextPrompt for UserInput {
    fn text(&self, msg: &str) -> Result<String, PromptError> {
        match Text::new(msg).prompt() {
            Ok(input) => Ok(input),
            Err(InquireError::OperationInterrupted | InquireError::OperationCanceled) => {
                Err(PromptError::Canceled)
            }
            Err(e) => Err(PromptError::InquireError(e)),
        }
    }
}
