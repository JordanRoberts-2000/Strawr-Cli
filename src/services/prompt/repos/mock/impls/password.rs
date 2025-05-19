use inquire::PasswordDisplayMode;

use crate::services::prompt::{
    mock::{MockInput, MockInputCall, MockInputRepo},
    traits::PasswordPrompt,
    user::UserInputError,
};

impl PasswordPrompt for MockInputRepo {
    fn password(
        &self,
        _display_mode: &PasswordDisplayMode,
        msg: &str,
    ) -> Result<String, UserInputError> {
        self.history.borrow_mut().push(MockInputCall::Password {
            msg: msg.to_string(),
        });

        let input = self
            .inputs
            .borrow_mut()
            .pop_front()
            .expect(format!("Ran out of test inputs for '{msg}'").as_str());

        match input {
            MockInput::Password(v) => Ok(v),
            _ => panic!("Expected Confirm input"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn password_returns_value_and_records_history() {
        let inputs = vec![
            MockInput::Password("secret1".into()),
            MockInput::Password("secret2".into()),
        ];
        let repo = MockInputRepo::from(inputs);
        assert!(repo.history.borrow().is_empty());

        // first call
        let got1 = repo
            .password(&PasswordDisplayMode::Hidden, "Enter password:")
            .unwrap();
        assert_eq!(got1, "secret1");

        // second call
        let got2 = repo
            .password(&PasswordDisplayMode::Hidden, "Enter password:")
            .unwrap();
        assert_eq!(got2, "secret2");

        // history recorded twice
        let hist = repo.history.borrow();
        assert_eq!(hist.len(), 2);
        assert_eq!(
            hist[0],
            MockInputCall::Password {
                msg: "Enter password:".to_string()
            }
        );
        assert_eq!(
            hist[1],
            MockInputCall::Password {
                msg: "Enter password:".to_string()
            }
        );
    }

    #[test]
    #[should_panic(expected = "Ran out of test inputs for 'Foo?'")]
    fn password_panics_if_no_inputs_left() {
        let repo = MockInputRepo::new(); // empty queue
        let _ = repo.password(&PasswordDisplayMode::Hidden, "Foo?");
    }

    #[test]
    #[should_panic(expected = "Expected Confirm input")]
    fn password_panics_on_wrong_variant() {
        // enqueue a non-Password input
        let repo = MockInputRepo::from(vec![MockInput::Text("oops".into())]);
        let _ = repo.password(&PasswordDisplayMode::Hidden, "Bar?");
    }
}
