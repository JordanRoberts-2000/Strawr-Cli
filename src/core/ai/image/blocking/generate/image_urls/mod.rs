mod builder {
    pub mod core;
    pub mod generate;
    pub mod validate_size;
}

use crate::ai::AiError;

pub use builder::core::ImageGenBuilder;

pub fn image(
    api_key: impl Into<String>,
    description: impl Into<String>,
) -> Result<ImageGenBuilder, AiError> {
    ImageGenBuilder::new(api_key, description)
}
