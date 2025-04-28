use inquire::{InquireError, Text};

use super::{Input, InputError, TestInput, UserInput};

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

impl TextInput for TestInput {
    fn text(&self, _msg: &str) -> Result<String, InputError> {
        let input = self
            .inputs
            .borrow_mut()
            .pop()
            .expect("Ran out of test inputs");

        match input {
            Input::Text(value) => Ok(value),
            _ => panic!("Expected Text input"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_input_returns_expected_value() {
        // Setup
        let test_inputs = vec![Input::Text("hello world".to_string())];
        let test_input = TestInput::new(test_inputs);

        // Act
        let result = test_input.text("Enter some text");

        // Assert
        assert_eq!(result.unwrap(), "hello world");
    }
}
