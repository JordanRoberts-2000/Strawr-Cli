use inquire::{InquireError, Select};

use crate::utils::input::{Input, TestInput, UserInput};

use super::{build_render_config, InputError};

pub trait SelectInput {
    fn select(&self, options: &[String], msg: &str) -> Result<String, InputError>;
}

impl SelectInput for UserInput {
    fn select(&self, options: &[String], msg: &str) -> Result<String, InputError> {
        let prompt = Select::new(msg, options.to_vec())
            .without_help_message()
            .with_page_size(4)
            .with_render_config(build_render_config())
            .prompt();

        match prompt {
            Ok(selected) => Ok(selected.to_string()),
            Err(InquireError::OperationInterrupted | InquireError::OperationCanceled) => {
                Err(InputError::Canceled)
            }
            Err(err) => Err(InputError::PromptError(err)),
        }
    }
}

impl SelectInput for TestInput {
    fn select(&self, _options: &[String], _msg: &str) -> Result<String, InputError> {
        let input = self
            .inputs
            .borrow_mut()
            .pop()
            .expect("Ran out of test inputs");

        match input {
            Input::Select(value) => Ok(value),
            _ => panic!("Expected Select input"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_input_returns_expected_value() {
        // Setup
        let test_inputs = vec![Input::Select("selected_option".to_string())];
        let test_input = TestInput::new(test_inputs);

        let options: Vec<String> = vec!["Option1", "Option2", "Option3"]
            .iter()
            .map(|s| s.to_string())
            .collect();

        // Act
        let result = test_input.select(&options, "Select an option");

        // Assert
        assert_eq!(result.unwrap(), "selected_option");
    }
}
