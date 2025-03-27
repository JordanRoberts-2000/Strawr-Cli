use reqwest::blocking::Client;
use serde_json::json;

use crate::{
    cli::commands::img::ImgError,
    constants::{KEYRING_OPEN_API_KEY, KEYRING_SERVICE},
    error::Result,
    services::keychain::keychain,
    utils::to_clipboard,
};

use super::GetManager;

impl GetManager {
    pub fn handle_alt(&mut self) -> Result<()> {
        let api_key = keychain(KEYRING_SERVICE, KEYRING_OPEN_API_KEY)?;
        let data_url = self
            .img
            .max_size(400)
            .webp()
            .map_err(ImgError::ImgFailed)?
            .data_url()
            .map_err(ImgError::ImgFailed)?;

        let request_body = json!({
            "model": "gpt-4o",
            "messages": [
                {
                    "role": "user",
                    "content": [
                        { "type": "text", "text": "Generate a concise descriptive alt text for the following image." },
                        { "type": "image_url", "image_url": { "url": data_url } }
                    ]
                }
            ]
        });

        let client = Client::new();
        let response = client
            .post("https://api.openai.com/v1/chat/completions")
            .bearer_auth(&api_key)
            .json(&request_body)
            .send()
            .map_err(ImgError::Request)?;

        let response_json: serde_json::Value = response.json().map_err(ImgError::Request)?;
        if let Some(content) = response_json["choices"][0]["message"]["content"].as_str() {
            println!("Alt Text: {}", content);
            to_clipboard(&content.to_string())?;
        } else {
            Err(ImgError::InvalidResponse(format!(
                "Failed to get alt text from response: {:?}",
                response_json
            )))?
        }

        Ok(())
    }
}
