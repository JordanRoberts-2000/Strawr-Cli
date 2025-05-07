use std::path::{Path, PathBuf};

use super::{command::TemplateInput, DEFAULT_FOLDER};

pub fn parse_template(s: &str) -> Result<TemplateInput, String> {
    if s.contains(':') {
        let parts: Vec<&str> = s.split(':').collect();
        let template = parts.get(0).copied().unwrap_or("").to_string();
        let variant = parts.get(1).map(|s| s.to_string());
        return Ok((template, variant));
    }

    Ok((s.to_string(), None))
}

pub struct Template {
    pub name: String,
    pub path: PathBuf,
    pub default_variant_path: PathBuf,
}

impl Template {
    pub fn new(name: &str, path: &Path) -> Self {
        let default_variant_path = path.join(DEFAULT_FOLDER);
        Self {
            name: name.to_string(),
            path: path.to_path_buf(),
            default_variant_path,
        }
    }
}

pub struct Variant {
    pub name: String,
    pub path: PathBuf,
}

impl Variant {
    pub fn new(name: &str, path: &Path) -> Self {
        Self {
            name: name.to_string(),
            path: path.to_path_buf(),
        }
    }
}
