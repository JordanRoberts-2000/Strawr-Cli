use reqwest::blocking::Client;
use serde_json::json;

pub fn alt_tag(api_key: &String, url: &String) -> Result<String, String> {
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
        .send()
        .map_err(|e| format!("Failed to send post request, err: {:?}", e))?;

    let response_json: serde_json::Value = response
        .json()
        .map_err(|e| format!("Failed parse response to json, err: {:?}", e))?;
    if let Some(content) = response_json["choices"][0]["message"]["content"].as_str() {
        Ok(content.to_string())
    } else {
        Err(format!(
            "Failed to retrieve alt tag from response json, res: {:?}",
            response_json
        ))
    }
}
