use super::PromptError;

pub trait CliInput: ConfirmPrompt + TextPrompt + SelectPrompt + SearchPrompt {}

impl<T> CliInput for T where T: ConfirmPrompt + TextPrompt + SelectPrompt + SearchPrompt {}

pub trait ConfirmPrompt {
    fn confirm(&self, msg: &str) -> Result<bool, PromptError>;
}

pub trait SelectPrompt {
    fn select(&self, options: &[String], msg: &str) -> Result<String, PromptError>;
}

pub trait SearchPrompt {
    fn search(&self, options: &[String], msg: &str) -> Result<String, PromptError>;
}

pub trait TextPrompt {
    fn text(&self, msg: &str) -> Result<String, PromptError>;
}
