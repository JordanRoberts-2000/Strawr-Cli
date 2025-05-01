use inquire::{InquireError, Select};

use crate::utils::input::{Input, TestInput, UserInput};

use super::{build_render_config, InputError};

pub trait SelectWithoutFilterInput {
    fn select_without_filter(&self, options: &[String], msg: &str) -> Result<String, InputError>;
}

impl SelectWithoutFilterInput for UserInput {
    fn select_without_filter(&self, options: &[String], msg: &str) -> Result<String, InputError> {
        let prompt = Select::new(msg, options.to_vec())
            .without_help_message()
            .with_page_size(4)
            .without_filtering()
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

impl SelectWithoutFilterInput for TestInput {
    fn select_without_filter(&self, _options: &[String], _msg: &str) -> Result<String, InputError> {
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
    fn test_select_without_filter_input_returns_expected_value() {
        // Setup
        let test_inputs = vec![Input::SelectWithoutFilter("selected_option".to_string())];
        let test_input = TestInput::new(test_inputs);

        let options: Vec<String> = vec!["Option1", "Option2", "Option3"]
            .iter()
            .map(|s| s.to_string())
            .collect();

        // Act
        let result = test_input.select_without_filter(&options, "Select an option");

        // Assert
        assert_eq!(result.unwrap(), "selected_option");
    }
}
