use crate::ai::{
    image::enums::{AiImageModel, Dalle2ImageSize, Dalle3ImageSize},
    AiResult,
};

use super::core::ImageGenBuilder;

impl ImageGenBuilder {
    pub fn validate_size(&self) -> AiResult<String> {
        let size = match self.model {
            AiImageModel::Dalle2 => Dalle2ImageSize::try_from(&self.size)?.to_string(),
            AiImageModel::Dalle3 => Dalle3ImageSize::try_from(&self.size)?.to_string(),
            AiImageModel::Custom(_) => self.size.to_string(),
        };

        Ok(size)
    }
}
