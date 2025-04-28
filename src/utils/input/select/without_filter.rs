use std::fmt::Display;

use inquire::{InquireError, Select};

use crate::utils::input::{Input, TestInput, UserInput};

use super::{build_render_config, InputError};

pub trait SelectWithoutFilterInput {
    fn select_without_filter<T>(&self, options: &Vec<T>, msg: &str) -> Result<String, InputError>
    where
        T: Display + Clone;
}

impl SelectWithoutFilterInput for UserInput {
    fn select_without_filter<T>(&self, options: &Vec<T>, msg: &str) -> Result<String, InputError>
    where
        T: Display + Clone,
    {
        let prompt = Select::new(msg, options.clone())
            .without_help_message()
            .without_filtering()
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

impl SelectWithoutFilterInput for TestInput {
    fn select_without_filter<T>(&self, _options: &Vec<T>, _msg: &str) -> Result<String, InputError>
    where
        T: Display + Clone,
    {
        let input = self
            .inputs
            .borrow_mut()
            .pop()
            .expect("Ran out of test inputs");

        match input {
            Input::SelectWithoutFilter(value) => Ok(value),
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

        let options = vec!["Option1", "Option2", "Option3"];

        // Act
        let result = test_input.select_without_filter(&options, "Select an option");

        // Assert
        assert_eq!(result.unwrap(), "selected_option");
    }
}
