use std::fmt;

#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Model {
    #[default]
    Dalle2,
    Dalle3,
    Custom(String),
}

#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ImageSize {
    Sm,
    Md,
    #[default]
    Lg,
    Tall,
    Wide,
    Custom((u32, u32)),
}

impl fmt::Display for ImageSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ImageSize::Sm => "256x256".to_string(),
            ImageSize::Md => "512x512".to_string(),
            ImageSize::Lg => "1024x1024".to_string(),
            ImageSize::Tall => "1024x1792".to_string(),
            ImageSize::Wide => "1792x1024".to_string(),
            ImageSize::Custom((w, h)) => format!("{}x{}", w, h),
        };
        write!(f, "{}", s)
    }
}
