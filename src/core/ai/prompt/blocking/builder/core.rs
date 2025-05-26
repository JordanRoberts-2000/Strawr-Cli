use crate::{ai::AiError, validation::validate};

pub struct PromptBuilder {
    pub(super) api_key: String,
    pub(super) prompt: String,
    pub(super) max_tokens: u16,
}

const DEFAULT_MAX_TOKENS: u16 = 200;

impl PromptBuilder {
    pub fn new(api_key: impl Into<String>, prompt: impl Into<String>) -> Result<Self, AiError> {
        let api_key = validate::not_empty(api_key.into())?;
        let prompt = validate::not_empty(prompt.into())?;

        Ok(Self {
            api_key,
            prompt,
            max_tokens: DEFAULT_MAX_TOKENS,
        })
    }

    pub fn max_tokens(mut self, max_tokens: u16) -> Self {
        self.max_tokens = max_tokens;
        self
    }
}
