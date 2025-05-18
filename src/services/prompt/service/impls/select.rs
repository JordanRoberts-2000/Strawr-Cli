use std::fmt::Display;

use crate::services::prompt::{PromptService, PromptServiceError};

impl PromptService {
    pub fn select<T: Display + Clone>(
        &self,
        options: &[T],
        msg: &str,
    ) -> Result<T, PromptServiceError> {
        let str_options: Vec<String> = options.iter().map(|opt| opt.to_string()).collect();

        let input = self
            .repo
            .select(&str_options, msg)
            .map_err(|e| PromptServiceError::RepoSelect(Box::new(e)))?;

        let idx = str_options
            .iter()
            .position(|s| s == &input)
            .ok_or_else(|| PromptServiceError::InvalidResponse(input.clone()))?;

        Ok(options[idx].clone())
    }
}
