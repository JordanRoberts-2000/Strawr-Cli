use crate::services::prompt::{
    mock::{MockInput, MockInputCall, MockInputRepo},
    traits::TextPrompt,
    user::UserInputError,
};

impl TextPrompt for MockInputRepo {
    fn text(&self, msg: &str) -> Result<String, UserInputError> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_returns_value_and_records_history() {
        let inputs = vec![
            MockInput::Text("hello".into()),
            MockInput::Text("world".into()),
        ];
        let repo = MockInputRepo::from(inputs);
        assert!(repo.history.borrow().is_empty());

        // first call
        let got = repo.text("Enter text:").unwrap();
        assert_eq!(got, "hello".to_string());
        // second call
        let got2 = repo.text("Enter text:").unwrap();
        assert_eq!(got2, "world".to_string());

        // history recorded twice
        let hist = repo.history.borrow();
        assert_eq!(hist.len(), 2);
        assert_eq!(
            hist[0],
            MockInputCall::Text {
                msg: "Enter text:".into()
            }
        );
        assert_eq!(
            hist[1],
            MockInputCall::Text {
                msg: "Enter text:".into()
            }
        );
    }

    #[test]
    #[should_panic(expected = "Ran out of test inputs for 'Foo?'")]
    fn text_panics_if_no_inputs_left() {
        let repo = MockInputRepo::new();
        let _ = repo.text("Foo?");
    }

    #[test]
    #[should_panic(expected = "Expected Text input")]
    fn text_panics_on_wrong_variant() {
        let repo = MockInputRepo::from(vec![MockInput::Confirm(true)]);
        let _ = repo.text("Bar?");
    }
}
