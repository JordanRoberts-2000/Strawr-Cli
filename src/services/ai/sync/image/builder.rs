use reqwest::blocking::Client;
use serde_json::json;

use crate::{error::ParseError, services::ai::AiError};

use super::models::{ImageSize, Model};

pub struct ImageBuilder {
    api_key: String,
    description: String,
    model: Model,
    size: ImageSize,
}

impl ImageBuilder {
    pub fn new<A, D>(api_key: A, description: D) -> Self
    where
        A: Into<String>,
        D: Into<String>,
    {
        Self {
            api_key: api_key.into(),
            description: description.into(),
            model: Model::default(),
            size: ImageSize::default(),
        }
    }

    pub fn size(mut self, size: ImageSize) -> Self {
        self.size = size;
        self
    }

    pub fn model(mut self, model: Model) -> Self {
        self.model = model;
        self
    }

    pub fn generate(&self) -> Result<String, AiError> {
        if self.description.trim().is_empty() {
            return Err(AiError::Validation("Description cannot be empty".into()));
        }

        if self.api_key.trim().is_empty() {
            return Err(AiError::Validation("API key cannot be empty".into()));
        }

        let request_body = json!({
            "prompt": self.description,
            "n": 1,
            "size": self.size.to_string(),
        });

        let open_ai_image_gen_url = "https://api.openai.com/v1/images/generations";
        let client = Client::new();

        let response = client
            .post(open_ai_image_gen_url)
            .bearer_auth(&self.api_key)
            .json(&request_body)
            .send()
            .map_err(AiError::RequestError)?;

        let response_json: serde_json::Value =
            response.json().map_err(|e| ParseError::JsonFetch {
                source: e,
                title: open_ai_image_gen_url.to_string(),
            })?;

        if let Some(data_array) = response_json.get("data").and_then(|v| v.as_array()) {
            if let Some(first_image) = data_array.first() {
                if let Some(url) = first_image.get("url").and_then(|v| v.as_str()) {
                    return Ok(url.to_string());
                } else {
                    return Err(AiError::InvalidJson {
                        json: first_image.clone(),
                        message: "Missing 'url' field in image object".to_string(),
                    });
                }
            } else {
                return Err(AiError::InvalidJson {
                    json: response_json.clone(),
                    message: "'data' array is empty".to_string(),
                });
            }
        } else {
            return Err(AiError::InvalidJson {
                json: response_json,
                message: "Missing or malformed 'data' array".to_string(),
            });
        }
    }
}
