use crate::services::prompt::{
    mock::{MockInput, MockInputCall, MockInputRepo},
    traits::SelectPrompt,
    user::UserInputError,
};

impl SelectPrompt for MockInputRepo {
    fn select(&self, options: &[String], msg: &str) -> Result<String, UserInputError> {
        self.history.borrow_mut().push(MockInputCall::Select {
            options: options.to_vec(),
            msg: msg.to_string(),
        });

        let input = self
            .inputs
            .borrow_mut()
            .pop_front()
            .expect(format!("Ran out of test inputs for '{msg}'").as_str());

        match input {
            MockInput::Select(value) => Ok(value),
            _ => panic!("Expected Select input"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn select_returns_value_and_records_history() {
        let options = vec!["a".to_string(), "b".to_string()];
        let inputs = vec![MockInput::Select("b".into()), MockInput::Select("a".into())];
        let repo = MockInputRepo::from(inputs);
        assert!(repo.history.borrow().is_empty());

        // first call
        let got1 = repo.select(&options, "Pick one:").unwrap();
        assert_eq!(got1, "b".to_string());
        // second call
        let got2 = repo.select(&options, "Pick one:").unwrap();
        assert_eq!(got2, "a".to_string());

        // history recorded correctly
        let hist = repo.history.borrow();
        assert_eq!(hist.len(), 2);
        assert_eq!(
            hist[0],
            MockInputCall::Select {
                options: options.clone(),
                msg: "Pick one:".into(),
            }
        );
        assert_eq!(
            hist[1],
            MockInputCall::Select {
                options: options.clone(),
                msg: "Pick one:".into(),
            }
        );
    }

    #[test]
    #[should_panic(expected = "Ran out of test inputs for 'Foo?'")]
    fn select_panics_if_no_inputs_left() {
        let repo = MockInputRepo::new();
        let _ = repo.select(&vec!["x".into(), "y".into()], "Foo?");
    }

    #[test]
    #[should_panic(expected = "Expected Select input")]
    fn select_panics_on_wrong_variant() {
        let repo = MockInputRepo::from(vec![MockInput::Confirm(true)]);
        let _ = repo.select(&vec!["x".into()], "Bar?");
    }
}
