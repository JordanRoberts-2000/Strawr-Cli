use std::fs;

use crate::{
    error::IoError,
    template::{
        constants::DEFAULT_FOLDER,
        utils::{Template, Variant},
        TemplateError, TemplateManager,
    },
    utils::validation,
};

impl<'a> TemplateManager<'a> {
    pub fn create_variant(&self, template: &Template, str: &str) -> Result<Variant, TemplateError> {
        let variant = validation::slug(&str).map_err(TemplateError::Validation)?;

        if variant == DEFAULT_FOLDER {
            return Err(TemplateError::Validation(format!(
                "'{DEFAULT_FOLDER}' is a reserved value"
            )));
        }

        let variant_path = template.path.join(&variant);

        if variant_path.exists() {
            return Err(TemplateError::VariantAlreadyExists(variant.clone()));
        }

        fs::create_dir_all(&variant_path)
            .map_err(|e| IoError::CreateDir(e, variant_path.clone()))?;

        Ok(Variant::new(&template, &variant, &variant_path))
    }
}
