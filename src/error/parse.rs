use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("failed to parse {title}:\n{source}")]
    Toml {
        source: toml::de::Error,
        title: String,
    },

    #[error("failed to parse {title}:\n{source}")]
    Json {
        source: serde_json::Error,
        title: String,
    },

    #[error("failed to serialize {title} to JSON:\n{source}")]
    JsonSerialize {
        source: serde_json::Error,
        title: String,
    },

    #[error("failed to fetch or decode JSON from {title}:\n{source}")]
    JsonFetch {
        source: reqwest::Error,
        title: String,
    },
}
