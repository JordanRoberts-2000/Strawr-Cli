use serde::Deserialize;

use crate::ai::AiError;

#[derive(Deserialize)]
pub struct ImageGenResponse {
    #[serde(rename = "data")]
    pub images: Vec<ImageData>,
}

#[derive(Deserialize)]
pub struct ImageData {
    pub url: String,
}

impl ImageGenResponse {
    pub fn url(self, raw: serde_json::Value) -> Result<String, AiError> {
        self.images
            .into_iter()
            .next()
            .map(|img| img.url)
            .ok_or_else(|| AiError::NoImagesReturned(raw))
    }

    pub fn urls(self, raw: serde_json::Value) -> Result<Vec<String>, AiError> {
        let urls: Vec<String> = self.images.into_iter().map(|img| img.url).collect();

        if urls.is_empty() {
            return Err(AiError::NoImagesReturned(raw));
        }

        Ok(urls)
    }
}
