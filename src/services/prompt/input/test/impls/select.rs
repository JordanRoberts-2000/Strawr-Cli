use crate::services::prompt::{traits::SelectPrompt, types::Input, PromptError, TestInput};

impl SelectPrompt for TestInput {
    fn select(&self, _options: &[String], msg: &str) -> Result<String, PromptError> {
        let input = self
            .inputs
            .borrow_mut()
            .pop_front()
            .expect(format!("Ran out of test inputs for '{msg}'").as_str());

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
        let test_input = TestInput::from(test_inputs);

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
