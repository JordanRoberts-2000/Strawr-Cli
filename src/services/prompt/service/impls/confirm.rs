use crate::services::prompt::{PromptService, PromptServiceError};

impl PromptService {
    pub fn confirm(&self, msg: &str) -> Result<bool, PromptServiceError> {
        self.repo
            .confirm(msg)
            .map_err(|e| PromptServiceError::RepoConfirm(Box::new(e)))
    }
}
