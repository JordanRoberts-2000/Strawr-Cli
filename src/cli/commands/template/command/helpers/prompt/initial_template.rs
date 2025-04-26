use inquire::InquireError;

use crate::utils::input;

pub fn prompt_create_initial_template() -> Result<bool, InquireError> {
    match input::confirm_action("No templates currently exist, would you like to create one?")
        .prompt()
    {
        Ok(input) => Ok(input),
        Err(InquireError::OperationInterrupted | InquireError::OperationCanceled) => Ok(false),
        Err(e) => Err(e),
    }
}
