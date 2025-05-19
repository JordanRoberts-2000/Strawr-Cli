use crate::services::prompt::{
    mock::{MockInput, MockInputCall, MockInputRepo},
    traits::SearchPrompt,
    user::UserInputError,
};

impl SearchPrompt for MockInputRepo {
    fn search(&self, options: &[String], msg: &str) -> Result<String, UserInputError> {
        self.history.borrow_mut().push(MockInputCall::Search {
            options: options.to_vec(),
            msg: msg.to_string(),
        });

        let input = self
            .inputs
            .borrow_mut()
            .pop_front()
            .expect(format!("Ran out of test inputs for '{msg}'").as_str());

        match input {
            MockInput::Search(value) => Ok(value),
            _ => panic!("Expected Select input"),
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_select_prompt_returns_expected_value() {
//         // Setup
//         let test_inputs = vec![Input::SelectWithoutFilter("selected_option".to_string())];
//         let test_input = TestInput::from(test_inputs);

//         let options: Vec<String> = vec!["Option1", "Option2", "Option3"]
//             .iter()
//             .map(|s| s.to_string())
//             .collect();

//         // Act
//         let result = test_input.search(&options, "Select an option");

//         // Assert
//         assert_eq!(result.unwrap(), "selected_option");
//     }
// }
