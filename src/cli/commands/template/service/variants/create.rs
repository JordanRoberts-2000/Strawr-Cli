use std::fs;

use crate::{
    cli::commands::template::{
        service::TemplateService,
        utils::{Template, Variant},
        TemplateError, DEFAULT_FOLDER,
    },
    error::IoError,
    utils::validation::slug,
};

impl<'a> TemplateService<'a> {
    pub fn create_variant(&self, template: &Template, str: &str) -> Result<Variant, TemplateError> {
        let variant = slug(&str).map_err(TemplateError::Validation)?;

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
