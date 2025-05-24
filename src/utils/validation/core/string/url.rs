use crate::utils::validation::ValidationError;
use url::Url;

/// Just syntax‐validate a URL.
pub fn url(input: impl AsRef<str>) -> Result<Url, ValidationError> {
    let s = input.as_ref();
    Url::parse(s).map_err(|_| ValidationError::InvalidUrl(s.to_string()))
}

/// Enforce “remote URL”: only http/https + must have host.
pub fn remote_url(input: impl AsRef<str>) -> Result<Url, ValidationError> {
    let s = input.as_ref();

    let url = url(s)?;

    let scheme = url.scheme();
    if scheme != "http" && scheme != "https" {
        return Err(ValidationError::UnsupportedScheme {
            input: s.to_string(),
            scheme: scheme.to_string(),
        });
    }

    if url.host().is_none() {
        return Err(ValidationError::MissingHost(s.to_string()));
    }

    Ok(url)
}
