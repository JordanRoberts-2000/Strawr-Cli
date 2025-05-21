use crate::services::prompt::{
    select_prompt,
    traits::SelectPrompt,
    user::{UserInputError, UserInputRepo},
};

impl SelectPrompt for UserInputRepo {
    fn select(&self, options: &[String], msg: &str) -> Result<String, UserInputError> {
        select_prompt(options, msg)
    }
}
