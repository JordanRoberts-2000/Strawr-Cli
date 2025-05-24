use reqwest::blocking::Client;
use serde_json::Value;

use crate::{ai::AiError, utils::parse_response};

pub struct OpenAiClient {
    inner: Client,
    key: String,
}

impl OpenAiClient {
    pub fn new(api_key: impl Into<String>) -> Self {
        OpenAiClient {
            inner: Client::new(),
            key: api_key.into(),
        }
    }

    pub fn post_json(&self, url: &str, body: &Value) -> Result<Value, AiError> {
        let response = self
            .inner
            .post(url)
            .bearer_auth(&self.key)
            .json(body)
            .send()
            .map_err(AiError::RequestError)?
            .error_for_status()
            .map_err(|e| {
                let status = e.status().unwrap_or_default();
                let url = e.url().map(|u| u.to_string()).unwrap_or_else(|| url.into());
                AiError::HttpStatus { status, url }
            })?;

        let json = parse_response(response)?;
        Ok(json)
    }
}
