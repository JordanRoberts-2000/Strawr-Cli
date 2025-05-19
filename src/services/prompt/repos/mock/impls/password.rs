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
