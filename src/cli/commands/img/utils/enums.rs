#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, clap::ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum ValidImageFormat {
    #[serde(alias = "jpg")]
    Jpeg,
    WebP,
    Png,
    Original,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, clap::ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum ImageSize {
    Sm,
    Md,
    Lg,
}

impl ImageSize {
    pub fn resolution(&self) -> &'static str {
        match self {
            ImageSize::Sm => "256x256",
            ImageSize::Md => "512x512",
            ImageSize::Lg => "1024x1024",
        }
    }
}
