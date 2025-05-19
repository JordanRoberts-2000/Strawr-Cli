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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_returns_value_and_records_history() {
        let options = vec!["apple".to_string(), "banana".to_string()];
        let inputs = vec![
            MockInput::Search("banana".into()),
            MockInput::Search("apple".into()),
        ];
        let repo = MockInputRepo::from(inputs);
        assert!(repo.history.borrow().is_empty());

        // first call
        let got1 = repo.search(&options, "Pick fruit:").unwrap();
        assert_eq!(got1, "banana".to_string());
        // second call
        let got2 = repo.search(&options, "Pick fruit:").unwrap();
        assert_eq!(got2, "apple".to_string());

        // history recorded twice
        let hist = repo.history.borrow();
        assert_eq!(hist.len(), 2);
        assert_eq!(
            hist[0],
            MockInputCall::Search {
                options: options.clone(),
                msg: "Pick fruit:".to_string(),
            }
        );
        assert_eq!(
            hist[1],
            MockInputCall::Search {
                options: options.clone(),
                msg: "Pick fruit:".to_string(),
            }
        );
    }

    #[test]
    #[should_panic(expected = "Ran out of test inputs for 'Foo?'")]
    fn search_panics_if_no_inputs_left() {
        let repo = MockInputRepo::new();
        let _ = repo.search(&vec!["x".into()], "Foo?");
    }

    #[test]
    #[should_panic(expected = "Expected Select input")]
    fn search_panics_on_wrong_variant() {
        let repo = MockInputRepo::from(vec![MockInput::Confirm(true)]);
        let _ = repo.search(&vec!["x".into()], "Bar?");
    }
}
