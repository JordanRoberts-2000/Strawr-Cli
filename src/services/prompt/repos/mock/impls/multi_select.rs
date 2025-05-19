use crate::services::prompt::{
    mock::{MockInput, MockInputCall, MockInputRepo},
    traits::MultiSelectPrompt,
    user::UserInputError,
};

impl MultiSelectPrompt for MockInputRepo {
    fn multi_select(
        &self,
        options: &[String],
        defaults: &[usize],
        msg: &str,
    ) -> Result<Vec<String>, UserInputError> {
        self.history.borrow_mut().push(MockInputCall::Checklist {
            msg: msg.to_string(),
            options: options.to_vec(),
            defaults: defaults.to_vec(),
        });

        let input = self
            .inputs
            .borrow_mut()
            .pop_front()
            .expect(format!("Ran out of test inputs for '{msg}'").as_str());

        match input {
            MockInput::Checklist(v) => Ok(v),
            _ => panic!("Expected Confirm input"),
        }
    }
}
