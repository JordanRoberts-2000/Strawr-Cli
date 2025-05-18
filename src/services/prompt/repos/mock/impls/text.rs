use std::convert::Infallible;

use crate::services::prompt::{
    mock::{MockInput, MockInputCall, MockInputRepo},
    traits::TextPrompt,
};

impl TextPrompt for MockInputRepo {
    type Error = Infallible;

    fn text(&self, msg: &str) -> Result<String, Infallible> {
        self.history.borrow_mut().push(MockInputCall::Text {
            msg: msg.to_string(),
        });

        let input = self
            .inputs
            .borrow_mut()
            .pop_front()
            .expect(format!("Ran out of test inputs for '{msg}'").as_str());

        match input {
            MockInput::Text(value) => Ok(value),
            _ => panic!("Expected Text input"),
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_text_input_returns_expected_value() {
//         // Setup
//         let test_inputs = vec![Input::Text("hello world".to_string())];
//         let test_input = TestInput::from(test_inputs);

//         // Act
//         let result = test_input.text("Enter some text");

//         // Assert
//         assert_eq!(result.unwrap(), "hello world");
//     }
// }
