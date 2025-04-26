use inquire::{InquireError, Text};

pub fn prompt_template_name() -> Result<Option<String>, InquireError> {
    match Text::new("Enter template name:").prompt() {
        Ok(value) => Ok(Some(value)),
        Err(InquireError::OperationCanceled | InquireError::OperationInterrupted) => Ok(None),
        Err(e) => Err(e),
    }
}
