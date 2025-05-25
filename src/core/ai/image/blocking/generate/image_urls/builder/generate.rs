use serde_json::json;

use crate::{
    ai::{
        image::{blocking::utils::OpenAiClient, models::ImageGenResponse},
        AiResult,
    },
    utils::parse_model,
};

use super::core::ImageGenBuilder;

const OPEN_AI_IMAGE_GEN_URL: &str = "https://api.openai.com/v1/images/generations";

impl ImageGenBuilder {
    pub fn generate(&self) -> AiResult<String> {
        let body = json!({
            "model": self.model.to_string(),
            "prompt": self.description,
            "n": 1,
            "size": self.validate_size()?,
        });

        let json = OpenAiClient::new(&self.api_key).post_json(OPEN_AI_IMAGE_GEN_URL, &body)?;
        let model: ImageGenResponse = parse_model(&json)?;

        model.url(json)
    }

    pub fn generate_multiple(&self, amount: &usize) -> AiResult<Vec<String>> {
        let body = json!({
            "model": self.model.to_string(),
            "prompt": self.description,
            "n": amount,
            "size": self.validate_size()?,
        });

        let json = OpenAiClient::new(&self.api_key).post_json(OPEN_AI_IMAGE_GEN_URL, &body)?;
        let model: ImageGenResponse = parse_model(&json)?;

        model.urls(json)
    }
}
