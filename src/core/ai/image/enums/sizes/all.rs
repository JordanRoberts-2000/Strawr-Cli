use std::str::FromStr;

use crate::ai::AiError;

use super::{dalle_2::Dalle2ImageSize, dalle_3::Dalle3ImageSize};

#[derive(Debug, Clone, Default, serde::Deserialize, strum_macros::Display)]
#[serde(rename_all = "lowercase")]
pub enum ImageSize {
    #[strum(to_string = "256x256")]
    Sm,
    #[strum(to_string = "512x512")]
    Md,
    #[default]
    #[strum(to_string = "1024x1024")]
    Lg,
    #[strum(to_string = "1024x1792")]
    Tall,
    #[strum(to_string = "1792x1024")]
    Wide,
    #[strum(to_string = "{width}x{height}")]
    Custom { width: u32, height: u32 },
}

impl From<Dalle2ImageSize> for ImageSize {
    fn from(d2: Dalle2ImageSize) -> Self {
        match d2 {
            Dalle2ImageSize::Sm => ImageSize::Sm,
            Dalle2ImageSize::Md => ImageSize::Md,
            Dalle2ImageSize::Lg => ImageSize::Lg,
            Dalle2ImageSize::Custom { width, height } => ImageSize::Custom { width, height },
        }
    }
}

impl From<Dalle3ImageSize> for ImageSize {
    fn from(d3: Dalle3ImageSize) -> Self {
        match d3 {
            Dalle3ImageSize::Lg => ImageSize::Lg,
            Dalle3ImageSize::Tall => ImageSize::Tall,
            Dalle3ImageSize::Wide => ImageSize::Wide,
            Dalle3ImageSize::Custom { width, height } => ImageSize::Custom { width, height },
        }
    }
}

impl FromStr for ImageSize {
    type Err = AiError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim().to_lowercase();
        match s.as_str() {
            "sm" | "small" => Ok(ImageSize::Sm),
            "md" | "medium" => Ok(ImageSize::Md),
            "lg" | "large" => Ok(ImageSize::Lg),
            "tall" => Ok(ImageSize::Tall),
            "wide" => Ok(ImageSize::Wide),
            other => {
                // try custom WxH
                let parts: Vec<_> = other.split('x').collect();
                if parts.len() != 2 {
                    return Err(AiError::ParseImageSize(other.to_string()));
                }
                let w = parts[0]
                    .parse::<u32>()
                    .map_err(|_| AiError::ParseImageSize(other.to_string()))?;
                let h = parts[1]
                    .parse::<u32>()
                    .map_err(|_| AiError::ParseImageSize(other.to_string()))?;
                Ok(ImageSize::Custom {
                    width: w,
                    height: h,
                })
            }
        }
    }
}
