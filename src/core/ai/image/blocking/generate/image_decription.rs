use serde_json::json;

use crate::{
    ai::{client::blocking::OpenAiClient, models::ChatCompletionResponse, AiResult},
    utils::parse_model,
};

const OPENAI_CHAT_URL: &str = "https://api.openai.com/v1/chat/completions";
const OPENAI_AI_MODEL: &str = "gpt-4o";

pub fn image_description(api_key: impl AsRef<str>, url: impl Into<String>) -> AiResult<String> {
    let body = json!({
      "model": OPENAI_AI_MODEL,
      "messages": [
          {
              "role": "user",
              "content": [
                  { "type": "text", "text": "Generate a concise descriptive alt text for the following image." },
                  { "type": "image_url", "image_url": { "url": url.into() } }
              ]
          }
      ]
    });

    let json = OpenAiClient::new(api_key.as_ref()).post_json(OPENAI_CHAT_URL, &body)?;

    let model: ChatCompletionResponse = parse_model(&json)?;

    model.first_message(json)
}
