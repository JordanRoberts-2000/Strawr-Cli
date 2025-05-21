use inquire::PasswordDisplayMode;

use crate::services::prompt::{
    password_prompt,
    traits::PasswordPrompt,
    user::{UserInputError, UserInputRepo},
};

impl PasswordPrompt for UserInputRepo {
    fn password(
        &self,
        display_mode: &PasswordDisplayMode,
        msg: &str,
    ) -> Result<String, UserInputError> {
        password_prompt(display_mode, msg)
    }
}
