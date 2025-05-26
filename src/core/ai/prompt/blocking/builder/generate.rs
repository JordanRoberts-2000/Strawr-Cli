use serde_json::json;

use crate::{
    ai::{client::blocking::OpenAiClient, models::ChatCompletionResponse, AiResult},
    utils::parse_model,
};

use super::core::PromptBuilder;

const OPENAI_CHAT_URL: &str = "https://api.openai.com/v1/chat/completions";
const OPENAI_AI_MODEL: &str = "gpt-3.5-turbo";

impl PromptBuilder {
    pub fn generate(&self) -> AiResult<String> {
        let body = json!({
            "model": OPENAI_AI_MODEL,
            "messages": [
                { "role": "user", "content": self.prompt.clone() }
            ],
            "temperature": 0.7,
            "max_tokens": self.max_tokens.clone()
        });

        let json = OpenAiClient::new(&self.api_key).post_json(OPENAI_CHAT_URL, &body)?;

        let model: ChatCompletionResponse = parse_model(&json)?;

        model.first_message(json)
    }
}
