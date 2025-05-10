use crate::services::prompt::{traits::ConfirmPrompt, types::Input, PromptError, TestInput};

impl ConfirmPrompt for TestInput {
    fn confirm(&self, msg: &str) -> Result<bool, PromptError> {
        let input = self
            .inputs
            .borrow_mut()
            .pop_front()
            .expect(format!("Ran out of test inputs for '{msg}'").as_str());

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
        let test_input = TestInput::from(test_inputs);

        let result = test_input.confirm("Are you sure?");
        assert_eq!(result.unwrap(), true);
    }

    #[test]
    fn test_confirm_no() {
        let test_inputs = vec![Input::Confirm(false)];
        let test_input = TestInput::from(test_inputs);

        let result = test_input.confirm("Proceed?");
        assert_eq!(result.unwrap(), false);
    }
}
