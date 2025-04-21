use inquire::{error::InquireError, Confirm};

pub fn confirm_action(prompt: &str) -> Result<bool, InquireError> {
    Confirm::new(prompt).with_default(false).prompt()
}
