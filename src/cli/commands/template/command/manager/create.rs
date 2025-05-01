use std::fs;

use crate::{
    cli::commands::template::{command::manager::TemplateManager, TemplateError, DEFAULT_FOLDER},
    error::io::IoError,
};

impl<'a> TemplateManager<'a> {
    pub fn create_template(
        &self,
        template: &str,
        variant: Option<&str>,
    ) -> Result<(), TemplateError> {
        match variant {
            Some(variant) => self.create_template_variant(&template, variant),
            None => self.create_initial_template(&template),
        }?;

        Ok(())
    }

    fn create_initial_template(&self, template: &str) -> Result<(), TemplateError> {
        let template_path = self.templates_path.join(template);

        if template_path.exists() {
            return Err(TemplateError::TemplateAlreadyExists {
                template: template.to_string(),
            });
        }

        let default_path = template_path.join(DEFAULT_FOLDER);

        fs::create_dir_all(&default_path)
            .map_err(|e| IoError::CreateDir(e, default_path.clone()))?;
        log::info!("Created new template '{}'", template);

        self.ctx.config.default_editor.open(&default_path)?;

        Ok(())
    }

    fn create_template_variant(&self, template: &str, variant: &str) -> Result<(), TemplateError> {
        let template_path = self.templates_path.join(template);

        if !template_path.exists() {
            return Err(TemplateError::TemplateNotFound {
                template: template.to_string(),
            });
        }

        let variant_path = template_path.join(&variant);

        if variant_path.exists() {
            return Err(TemplateError::VariantAlreadyExists {
                variant: variant.to_string(),
            });
        }

        fs::create_dir(&variant_path).map_err(|e| IoError::CreateDir(e, variant_path.clone()))?;
        log::info!("Created variant '{variant}' for template '{template}'");

        self.ctx.config.default_editor.open(&variant_path)?;

        Ok(())
    }
}
