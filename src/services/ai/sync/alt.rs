use reqwest::blocking::Client;
use serde_json::json;

use crate::services::ai::error::{Error, Result};

pub fn alt_tag(api_key: &String, url: &String) -> Result<String> {
    let request_body = json!({
      "model": "gpt-4o",
      "messages": [
          {
              "role": "user",
              "content": [
                  { "type": "text", "text": "Generate a concise descriptive alt text for the following image." },
                  { "type": "image_url", "image_url": { "url": url } }
              ]
          }
      ]
    });

    let client = Client::new();
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(&api_key)
        .json(&request_body)
        .send()?;

    let response_json: serde_json::Value = response.json()?;
    if let Some(content) = response_json["choices"][0]["message"]["content"].as_str() {
        Ok(content.to_string())
    } else {
        Err(Error::InvalidJson {
            json: response_json,
            message: "Expected structure choices[0].message.content not found".to_string(),
        })
    }
}
