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

    #[error("failed to decode JSON from {url}:\n{source}")]
    JsonResponse {
        source: reqwest::Error,
        url: reqwest::Url,
    },

    #[error("failed to parse JSON into {model}:\nerror: {source}\njson: {source}")]
    JsonModel {
        source: serde_json::Error,
        json: String,
        model: String,
    },
}
