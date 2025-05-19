use std::fmt::Display;

use crate::services::prompt::{PromptService, PromptServiceError};

impl PromptService {
    pub fn multi_select<T: Display + Clone>(
        &self,
        options: &[T],
        msg: &str,
    ) -> Result<Vec<T>, PromptServiceError> {
        let str_options: Vec<String> = options.iter().map(|opt| opt.to_string()).collect();

        let selected_strs = self.repo.multi_select(&str_options, &[], msg)?;

        let mut result = Vec::with_capacity(selected_strs.len());
        for sel in selected_strs {
            let idx = str_options
                .iter()
                .position(|s| s == &sel)
                .ok_or_else(|| PromptServiceError::InvalidResponse(sel.clone()))?;
            result.push(options[idx].clone());
        }

        Ok(result)
    }

    pub fn multi_select_with_defaults<T: Display + Clone>(
        &self,
        options: &[T],
        defaults: &[usize],
        msg: &str,
    ) -> Result<Vec<T>, PromptServiceError> {
        let str_options: Vec<String> = options.iter().map(|opt| opt.to_string()).collect();

        let selected_strs = self.repo.multi_select(&str_options, defaults, msg)?;

        let mut result = Vec::with_capacity(selected_strs.len());
        for sel in selected_strs {
            let idx = str_options
                .iter()
                .position(|s| s == &sel)
                .ok_or_else(|| PromptServiceError::InvalidResponse(sel.clone()))?;
            result.push(options[idx].clone());
        }

        Ok(result)
    }
}
