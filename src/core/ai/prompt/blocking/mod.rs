mod builder {
    pub mod core;
    mod generate;
}

pub use builder::core::PromptBuilder;

use crate::ai::AiError;

pub fn prompt(api_key: &str, prompt: &str) -> Result<PromptBuilder, AiError> {
    PromptBuilder::new(api_key, prompt)
}
