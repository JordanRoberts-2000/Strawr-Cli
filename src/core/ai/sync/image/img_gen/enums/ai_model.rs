#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AiImageModel {
    #[default]
    Dalle2,
    Dalle3,
    Custom(String),
}
