use {std::path::PathBuf, strum::VariantNames};

use crate::utils::validation::validate as v;

pub mod validate {
    use url::Url;

    use super::*;

    pub fn existing_dir(str: &str) -> Result<PathBuf, String> {
        v::existing_dir(str).map_err(|e| e.to_string())
    }

    pub fn existing_file(str: &str) -> Result<PathBuf, String> {
        v::existing_file(str).map_err(|e| e.to_string())
    }

    pub fn reserved<E: VariantNames>(str: &str) -> Result<String, String> {
        v::reserved::<E>(str).map_err(|e| e.to_string())
    }

    pub fn slug(str: &str) -> Result<String, String> {
        v::slug(str).map_err(|e| e.to_string())
    }

    pub fn url(str: &str) -> Result<Url, String> {
        v::url(str).map_err(|e| e.to_string())
    }

    pub fn remote_url(str: &str) -> Result<Url, String> {
        v::remote_url(str).map_err(|e| e.to_string())
    }

    pub fn not_empty(str: &str) -> Result<String, String> {
        v::not_empty(str).map_err(|e| e.to_string())
    }
}
