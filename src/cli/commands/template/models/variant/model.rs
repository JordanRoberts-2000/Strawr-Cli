use std::path::PathBuf;

use crate::{
    template::{constants::DEFAULT_FOLDER, models::Template, TemplateError},
    utils::validation,
};

pub struct Variant {
    pub name: String,
    pub path: PathBuf,
    pub template_name: String,
    pub template_path: PathBuf,
}

impl Variant {
    pub fn new(template: &Template, name: &str) -> Self {
        let path = template.path.join(&name);
        Self {
            name: name.to_string(),
            path,
            template_name: template.name.clone(),
            template_path: template.path.clone(),
        }
    }

    pub fn validate_name(name: &str) -> Result<String, TemplateError> {
        let variant = validation::slug(name).map_err(TemplateError::Validation)?;

        if variant == DEFAULT_FOLDER {
            return Err(TemplateError::Validation(format!(
                "'{DEFAULT_FOLDER}' is a reserved value"
            )));
        }

        Ok(variant)
    }
}
