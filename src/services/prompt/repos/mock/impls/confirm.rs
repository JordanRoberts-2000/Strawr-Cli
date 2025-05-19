use crate::services::prompt::{
    mock::{MockInput, MockInputCall, MockInputRepo},
    traits::ConfirmPrompt,
    user::UserInputError,
};

impl ConfirmPrompt for MockInputRepo {
    fn confirm(&self, msg: &str) -> Result<bool, UserInputError> {
        self.history.borrow_mut().push(MockInputCall::Confirm {
            msg: msg.to_string(),
        });

        let input = self
            .inputs
            .borrow_mut()
            .pop_front()
            .expect(format!("Ran out of test inputs for '{msg}'").as_str());

        match input {
            MockInput::Confirm(v) => Ok(v),
            _ => panic!("Expected Confirm input"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn confirm_returns_value_and_records_history() {
        let inputs = vec![MockInput::Confirm(true), MockInput::Confirm(false)];
        let repo = MockInputRepo::from(inputs);
        assert!(repo.history.borrow().is_empty());

        // returns input
        let got = repo.confirm("Proceed?").unwrap();
        assert_eq!(got, true);

        let got = repo.confirm("Proceed?").unwrap();
        assert_eq!(got, false);

        // records history
        let hist = repo.history.borrow();
        assert_eq!(hist.len(), 2);

        assert_eq!(
            hist[0],
            MockInputCall::Confirm {
                msg: "Proceed?".to_string()
            }
        );
    }

    #[test]
    #[should_panic(expected = "Ran out of test inputs for 'Foo?'")]
    fn confirm_panics_if_no_inputs_left() {
        let repo = MockInputRepo::new(); // empty queue
        let _ = repo.confirm("Foo?");
    }

    #[test]
    #[should_panic(expected = "Expected Confirm input")]
    fn confirm_panics_on_wrong_variant() {
        let repo = MockInputRepo::from(vec![MockInput::Text("oops".into())]);
        // should panic with “Expected Confirm input”
        let _ = repo.confirm("Bar?");
    }
}
