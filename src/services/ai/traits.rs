use crate::ai::{AiImageModel, ImageSize};

use super::AiServiceError;

pub trait GenImage {
    fn gen_image(
        &self,
        description: &str,
        model: AiImageModel,
        size: ImageSize,
    ) -> Result<String, AiServiceError>;

    fn get_image_description(&self, url: &str) -> Result<String, AiServiceError>;
}

pub trait Prompt {
    fn prompt(&self, prompt: &str, max_tokens: u16) -> Result<String, AiServiceError>;
}
