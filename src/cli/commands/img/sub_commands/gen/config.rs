use crate::services::ai::sync::{ImageSize, Model};

#[derive(Debug, serde::Deserialize, validator::Validate, Clone)]
pub struct ImgGenConfig {
    pub default_ai_model: Model,
    pub default_img_size: ImageSize,
}
