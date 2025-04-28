use inquire::{Confirm, InquireError};

use super::{Input, InputError, TestInput, UserInput};

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

impl ConfirmInput for TestInput {
    fn confirm(&self, _msg: &str) -> Result<bool, InputError> {
        let input = self
            .inputs
            .borrow_mut()
            .pop()
            .expect("Ran out of test inputs");

        match input {
            Input::Confirm(v) => Ok(v),
            _ => panic!("Expected Confirm input"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_confirm_yes() {
        let test_inputs = vec![Input::Confirm(true)];
        let test_input = TestInput::new(test_inputs);

        let result = test_input.confirm("Are you sure?");
        assert_eq!(result.unwrap(), true);
    }

    #[test]
    fn test_confirm_no() {
        let test_inputs = vec![Input::Confirm(false)];
        let test_input = TestInput::new(test_inputs);

        let result = test_input.confirm("Proceed?");
        assert_eq!(result.unwrap(), false);
    }
}
