use inquire::{InquireError, MultiSelect};

use crate::services::prompt::{
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
        let prompt = MultiSelect::new(msg, options.to_vec())
            .with_default(defaults)
            .prompt();

        match prompt {
            Ok(input) => Ok(input),
            Err(InquireError::OperationInterrupted | InquireError::OperationCanceled) => {
                Err(UserInputError::Canceled)
            }
            Err(e) => Err(UserInputError::InquireError(e)),
        }
    }
}
