use std::str::FromStr;

use crate::commands::img::ImgCmdError;

#[derive(Debug, Clone, PartialEq)]
pub enum ImageSize {
    Sm,
    Md,
    Lg,
    Xl,
    Pixels(u32),
}

impl FromStr for ImageSize {
    type Err = ImgCmdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let low = s.to_ascii_lowercase();
        match low.as_str() {
            "sm" | "small" => Ok(Self::Sm),
            "md" | "medium" => Ok(Self::Md),
            "lg" | "large" => Ok(Self::Lg),
            "xl" => Ok(Self::Xl),
            other => {
                // try to parse as raw pixels
                other
                    .parse::<u32>()
                    .map(Self::Pixels)
                    .map_err(|_| ImgCmdError::ParseImageSize(s.to_owned()))
            }
        }
    }
}

impl ImageSize {
    pub fn to_pixels(&self) -> u32 {
        match *self {
            ImageSize::Sm => 640,
            ImageSize::Md => 768,
            ImageSize::Lg => 1024,
            ImageSize::Xl => 1280,
            ImageSize::Pixels(n) => n,
        }
    }
}
