use crate::ai::{image::enums::ImageSize, AiError};

#[derive(Debug, Clone, serde::Deserialize, strum_macros::Display)]
#[serde(rename_all = "lowercase")]
pub enum Dalle3ImageSize {
    #[strum(to_string = "1024x1024")]
    Lg,
    #[strum(to_string = "1024x1792")]
    Tall,
    #[strum(to_string = "1792x1024")]
    Wide,
    #[strum(to_string = "{width}x{height}")]
    Custom { width: u32, height: u32 },
}

impl TryFrom<&ImageSize> for Dalle3ImageSize {
    type Error = AiError;
    fn try_from(sz: &ImageSize) -> Result<Self, Self::Error> {
        match sz {
            ImageSize::Lg => Ok(Dalle3ImageSize::Lg),
            ImageSize::Tall => Ok(Dalle3ImageSize::Tall),
            ImageSize::Wide => Ok(Dalle3ImageSize::Wide),
            ImageSize::Custom { width, height } => Ok(Dalle3ImageSize::Custom {
                width: *width,
                height: *height,
            }),
            other => Err(AiError::UnsupportedSize {
                size: other.to_string(),
                model: "Dalle-3".to_string(),
            }),
        }
    }
}
