use crate::ai::{image::enums::ImageSize, AiError};
use std::convert::TryFrom;

#[derive(Debug, Clone, serde::Deserialize, strum_macros::Display)]
#[serde(rename_all = "lowercase")]
pub enum Dalle2ImageSize {
    #[strum(to_string = "256x256")]
    Sm,
    #[strum(to_string = "512x512")]
    Md,
    #[strum(to_string = "1024x1024")]
    Lg,
    #[strum(to_string = "{width}x{height}")]
    Custom { width: u32, height: u32 },
}

impl TryFrom<&ImageSize> for Dalle2ImageSize {
    type Error = AiError;

    fn try_from(sz: &ImageSize) -> Result<Self, Self::Error> {
        match sz {
            ImageSize::Sm => Ok(Dalle2ImageSize::Sm),
            ImageSize::Md => Ok(Dalle2ImageSize::Md),
            ImageSize::Lg => Ok(Dalle2ImageSize::Lg),
            ImageSize::Custom { width, height } => Ok(Dalle2ImageSize::Custom {
                width: *width,
                height: *height,
            }),
            other => Err(AiError::UnsupportedSize {
                size: other.to_string(),
                model: "Dalle-2".to_string(),
            }),
        }
    }
}
