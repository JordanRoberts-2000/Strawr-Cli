use serde::Deserialize;

use super::AiError;

#[derive(Deserialize)]
pub struct ChatCompletionResponse {
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

impl ChatCompletionResponse {
    /// Pulls the first choice’s content, or returns an error if no choices returned.
    pub fn first_message(self, raw: serde_json::Value) -> Result<String, AiError> {
        self.choices
            .into_iter()
            .next()
            .map(|c| c.message.content)
            .ok_or_else(|| AiError::NoContentReturned(raw))
    }
}
