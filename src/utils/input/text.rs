use inquire::{InquireError, Text};

use super::{InputError, UserInput};

pub trait TextInput {
    fn text(&self, msg: &str) -> Result<String, InputError>;
}

impl TextInput for UserInput {
    fn text(&self, msg: &str) -> Result<String, InputError> {
        match Text::new(msg).prompt() {
            Ok(input) => Ok(input),
            Err(InquireError::OperationInterrupted | InquireError::OperationCanceled) => {
                Err(InputError::Canceled)
            }
            Err(e) => Err(InputError::PromptError(e)),
        }
    }
}
