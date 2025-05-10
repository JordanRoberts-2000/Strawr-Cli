use crate::services::prompt::{enums::Input, traits::TextPrompt, PromptError, TestInput};

impl TextPrompt for TestInput {
    fn text(&self, msg: &str) -> Result<String, PromptError> {
        let input = self
            .inputs
            .borrow_mut()
            .pop_front()
            .expect(format!("Ran out of test inputs for '{msg}'").as_str());

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
        let test_input = TestInput::from(test_inputs);

        // Act
        let result = test_input.text("Enter some text");

        // Assert
        assert_eq!(result.unwrap(), "hello world");
    }
}
