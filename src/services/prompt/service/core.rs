use crate::services::prompt::{traits::*, user::UserInputRepo};

use inquire::PasswordDisplayMode;

pub trait PromptRepo:
    ConfirmPrompt + TextPrompt + SelectPrompt + SearchPrompt + MultiSelectPrompt + PasswordPrompt
{
}

impl<T> PromptRepo for T where
    T: ConfirmPrompt
        + TextPrompt
        + SelectPrompt
        + SearchPrompt
        + MultiSelectPrompt
        + PasswordPrompt
{
}

pub struct PromptService {
    pub(super) repo: Box<dyn PromptRepo>,
    pub(super) password_display: PasswordDisplayMode,
}

impl PromptService {
    pub fn new() -> Self {
        Self {
            repo: Box::new(UserInputRepo),
            password_display: PasswordDisplayMode::Hidden,
        }
    }

    pub fn set_password_mode(&mut self, password_display: &PasswordDisplayMode) -> &mut Self {
        self.password_display = *password_display;
        self
    }

    pub fn set_repo(&mut self, repo: impl PromptRepo + 'static) -> &mut Self {
        self.repo = Box::new(repo);
        self
    }
}
