use serde::Deserialize;

use crate::ai::AiError;

#[derive(Deserialize)]
pub struct ImageDescriptionResponse {
    pub choices: Vec<Choice>,
}

#[derive(Deserialize)]
pub struct Choice {
    pub message: Message,
}

#[derive(Deserialize)]
pub struct Message {
    pub content: String,
}

impl ImageDescriptionResponse {
    pub fn description(self, raw: serde_json::Value) -> Result<String, AiError> {
        self.choices
            .into_iter()
            .next()
            .map(|c| c.message.content)
            .ok_or_else(|| AiError::NoContentReturned(raw))
    }
}
