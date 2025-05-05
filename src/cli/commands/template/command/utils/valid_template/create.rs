use std::fs;

use crate::{
    cli::commands::template::{TemplateError, DEFAULT_FOLDER},
    error::IoError,
    utils::validation::slug,
};

use super::Template;

impl Template {
    fn create_default(&self) -> Result<(), TemplateError> {
        if self.path.exists() {
            return Err(TemplateError::Validation(format!(
                "Template '{}' already exists",
                self.name
            )));
        }

        let default_path = &self.path.join(DEFAULT_FOLDER);

        fs::create_dir_all(&default_path)
            .map_err(|e| IoError::CreateDir(e, default_path.clone()))?;

        Ok(())
    }

    fn create_variant(&self, variant: &str) -> Result<(), TemplateError> {
        let valid_slug = slug(&variant).map_err(TemplateError::Validation)?;

        if valid_slug == DEFAULT_FOLDER {
            return Err(TemplateError::Validation(format!(
                "'{DEFAULT_FOLDER}' is a reserved value"
            )));
        }

        if !(self.path.join(DEFAULT_FOLDER).exists()) {
            return Err(TemplateError::TemplateNotFound(self.name.clone()));
        }

        let path = self.path.join(&valid_slug);

        if path.exists() {
            return Err(TemplateError::Validation(format!(
                "Variant '{}' already exists",
                variant
            )));
        }

        fs::create_dir_all(&path).map_err(|e| IoError::CreateDir(e, path.clone()))?;

        Ok(())
    }

    pub fn create(&self, variant: &Option<&str>) -> Result<Self, TemplateError> {
        match variant {
            None => self.create_default()?,
            Some(v) => self.create_variant(v)?,
        };

        Ok(self.clone())
    }
}
