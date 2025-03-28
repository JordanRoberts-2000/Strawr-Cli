use reqwest::blocking::Client;
use serde_json::json;

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

    pub fn generate(&self) -> Result<String, String> {
        if self.description.is_empty() {
            return Err("Description cannot be empty".to_string());
        }

        if self.api_key.is_empty() {
            return Err("API key cannot be empty".to_string());
        }

        let request_body = json!({
            "prompt": self.description,
            "n": 1,
            "size": self.size.to_string(),
        });

        let client = Client::new();
        let response = client
            .post("https://api.openai.com/v1/images/generations")
            .bearer_auth(&self.api_key)
            .json(&request_body)
            .send()
            .map_err(|e| format!("Failed to send post request, err: {}", e))?;

        let response_json: serde_json::Value = response
            .json()
            .map_err(|e| format!("Failed to parse response to json, err: {}", e))?;

        if let Some(data_array) = response_json.get("data").and_then(|v| v.as_array()) {
            if let Some(first_image) = data_array.first() {
                if let Some(url) = first_image.get("url").and_then(|v| v.as_str()) {
                    return Ok(url.to_string());
                } else {
                    return Err("No 'url' field found in the image response".to_string());
                }
            } else {
                return Err("The 'data' array is empty".to_string());
            }
        } else {
            return Err(format!("No image data returned: {:#?}", response_json));
        }
    }
}
