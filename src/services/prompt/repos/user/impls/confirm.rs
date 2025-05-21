use crate::services::prompt::{
    confirm_prompt,
    traits::ConfirmPrompt,
    user::{UserInputError, UserInputRepo},
};

impl ConfirmPrompt for UserInputRepo {
    fn confirm(&self, msg: &str) -> Result<bool, UserInputError> {
        confirm_prompt(msg)
    }
}
