use crate::services::prompt::{PromptService, PromptServiceError};

impl PromptService {
    pub fn text(&self, msg: &str) -> Result<String, PromptServiceError> {
        let input = self.repo.text(msg)?;
        Ok(input)
    }
}
