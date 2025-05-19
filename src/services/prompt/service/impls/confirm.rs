use crate::services::prompt::{PromptService, PromptServiceError};

impl PromptService {
    pub fn confirm(&self, msg: &str) -> Result<bool, PromptServiceError> {
        let input = self.repo.confirm(msg)?;
        Ok(input)
    }
}
