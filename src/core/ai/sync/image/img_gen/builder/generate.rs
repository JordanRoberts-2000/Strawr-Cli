use serde_json::json;

use crate::{
    ai::{open_ai_client::OpenAiClient, sync::r#gen::ImageGenResponse, AiError},
    utils::parse_model,
};

use super::core::ImageGenBuilder;

const OPEN_AI_IMAGE_GEN_URL: &str = "https://api.openai.com/v1/images/generations";

impl ImageGenBuilder {
    pub fn generate(&self) -> Result<String, AiError> {
        let body = json!({
            "prompt": self.description,
            "n": 1,
            "size": self.size.to_string(),
        });

        let json = OpenAiClient::new(&self.api_key).post_json(OPEN_AI_IMAGE_GEN_URL, &body)?;
        let model: ImageGenResponse = parse_model(&json)?;

        model.url(json)
    }
}
