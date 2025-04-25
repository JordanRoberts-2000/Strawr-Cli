use std::fs;

use crate::{
    cli::commands::template::{command::manager::TemplateManager, TemplateError},
    error::io::IoError,
    utils::fs::subfolders,
};

impl<'a> TemplateManager<'a> {
    pub fn create_template(
        &self,
        template: &str,
        variant: &Option<String>,
    ) -> Result<(), TemplateError> {
        let template_path = self.templates_path.join(template);
        let existing_templates =
            subfolders(&template_path).map_err(TemplateError::NoExistingTemplate)?;

        match variant {
            Some(variant_name) => {
                if !existing_templates.contains(&template.to_string()) {
                    return Err(TemplateError::CreatingVariantWithoutDefault);
                }

                let current_variants =
                    subfolders(&template_path).map_err(TemplateError::NoExistingTemplate)?;

                if current_variants.contains(variant_name) {
                    return Err(TemplateError::VariantAlreadyExists);
                }

                let variant_path = template_path.join(variant_name);

                fs::create_dir(&variant_path)
                    .map_err(|e| IoError::CreateDir(e, variant_path.clone()))?;
                log::info!("Created variant '{variant_name}' for template '{template}'");

                self.ctx.config.default_editor.open(&variant_path)?;
            }
            None => {
                if existing_templates.contains(&template.to_string()) {
                    return Err(TemplateError::TemplateAlreadyExists);
                }

                let default_path = template_path.join("default");

                fs::create_dir_all(&default_path)
                    .map_err(|e| IoError::CreateDir(e, default_path.clone()))?;
                log::info!("Created new template '{}'", template);

                self.ctx.config.default_editor.open(&default_path)?;
            }
        }

        Ok(())
    }
}
