use crate::services::prompt::{
    multi_select_prompt,
    traits::MultiSelectPrompt,
    user::{UserInputError, UserInputRepo},
};

impl MultiSelectPrompt for UserInputRepo {
    fn multi_select(
        &self,
        options: &[String],
        defaults: &[usize],
        msg: &str,
    ) -> Result<Vec<String>, UserInputError> {
        multi_select_prompt(options, defaults, msg)
    }
}
