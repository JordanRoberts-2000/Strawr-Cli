use std::fs::rename;

use crate::{
    error::IoError,
    template::{constants::DEFAULT_FOLDER, utils::Variant, TemplateError, TemplateService},
    utils::validation::slug,
};

impl<'a> TemplateService<'a> {
    pub fn rename_variant(&self, variant: &Variant, to: &str) -> Result<Variant, TemplateError> {
        let valid_slug = slug(&to).map_err(TemplateError::Validation)?;

        if valid_slug == DEFAULT_FOLDER {
            return Err(TemplateError::Validation(format!(
                "'{DEFAULT_FOLDER}' is a reserved value"
            )));
        }

        let new_variant_path = variant.template_path.join(&valid_slug);

        if new_variant_path.exists() {
            return Err(TemplateError::Validation(format!(
                "Rename failed as '{}' is an existing variant",
                variant.name
            )));
        }

        rename(&variant.path, &new_variant_path)
            .map_err(|e| IoError::Rename(e, variant.path.clone(), new_variant_path.clone()))?;

        Ok(Variant {
            name: valid_slug,
            path: new_variant_path,
            template_name: variant.name.clone(),
            template_path: variant.path.clone(),
        })
    }
}
