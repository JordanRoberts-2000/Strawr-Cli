use crate::services::prompt::traits::*;

pub trait PromptRepo:
    ConfirmPrompt + TextPrompt + SelectPrompt + SearchPrompt + MultiSelectPrompt
{
}

impl<T> PromptRepo for T where
    T: ConfirmPrompt + TextPrompt + SelectPrompt + SearchPrompt + MultiSelectPrompt
{
}

pub struct PromptService {
    pub(super) repo: Box<dyn PromptRepo>,
}

impl PromptService {
    pub fn new(repo: impl PromptRepo + 'static) -> Self {
        Self {
            repo: Box::new(repo),
        }
    }
}
