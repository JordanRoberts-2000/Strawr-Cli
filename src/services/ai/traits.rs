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
