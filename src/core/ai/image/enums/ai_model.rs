use strum_macros::Display;

#[derive(Debug, Clone, Default, serde::Deserialize, Display)]
#[serde(rename_all = "lowercase")]
pub enum AiImageModel {
    #[default]
    #[strum(to_string = "dall-e-2")]
    Dalle2,
    #[strum(to_string = "dall-e-3")]
    Dalle3,
    #[strum(to_string = "{0}")]
    Custom(String),
}
