use crate::services::prompt::{
    search_prompt,
    traits::SearchPrompt,
    user::{UserInputError, UserInputRepo},
};

impl SearchPrompt for UserInputRepo {
    fn search(&self, options: &[String], msg: &str) -> Result<String, UserInputError> {
        search_prompt(options, msg)
    }
}
