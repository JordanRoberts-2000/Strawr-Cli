use crate::services::prompt::{PromptService, PromptServiceError};

impl PromptService {
    pub fn password(&self, msg: &str) -> Result<String, PromptServiceError> {
        let input = self.repo.password(&self.password_display, msg)?;
        Ok(input)
    }
}
