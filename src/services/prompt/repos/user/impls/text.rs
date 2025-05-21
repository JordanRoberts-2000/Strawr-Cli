use crate::services::prompt::{
    text_prompt,
    traits::TextPrompt,
    user::{UserInputError, UserInputRepo},
};

impl TextPrompt for UserInputRepo {
    fn text(&self, msg: &str) -> Result<String, UserInputError> {
        text_prompt(msg)
    }
}
