use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("failed to parse {1}\n{0}")]
    Toml(toml::de::Error, String),

    #[error("failed to parse {1}\n{0}")]
    Json(serde_json::Error, String),
}
