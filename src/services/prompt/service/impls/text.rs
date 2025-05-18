use crate::services::prompt::{PromptService, PromptServiceError};

impl PromptService {
    pub fn text(&self, msg: &str) -> Result<String, PromptServiceError> {
        self.repo
            .text(msg)
            .map_err(|e| PromptServiceError::RepoText(Box::new(e)))
    }
}
