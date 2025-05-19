use super::user::UserInputError;

pub trait ConfirmPrompt {
    fn confirm(&self, msg: &str) -> Result<bool, UserInputError>;
}

pub trait SelectPrompt {
    fn select(&self, options: &[String], msg: &str) -> Result<String, UserInputError>;
}

pub trait SearchPrompt {
    fn search(&self, options: &[String], msg: &str) -> Result<String, UserInputError>;
}

pub trait TextPrompt {
    fn text(&self, msg: &str) -> Result<String, UserInputError>;
}
