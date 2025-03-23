#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, clap::ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum ValidImageFormat {
    #[serde(alias = "jpg")]
    Jpeg,
    WebP,
    Png,
    Original,
}
