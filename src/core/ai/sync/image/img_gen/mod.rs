use builder::core::ImageGenBuilder;

mod builder {
    pub mod core;
    pub mod generate;
}
mod enums {
    pub mod ai_model;
    pub mod image_size;
}

use crate::ai::AiError;

pub use enums::{ai_model::AiImageModel, image_size::ImageSize};

pub fn image(
    api_key: impl Into<String>,
    description: impl Into<String>,
) -> Result<ImageGenBuilder, AiError> {
    ImageGenBuilder::new(api_key, description)
}
