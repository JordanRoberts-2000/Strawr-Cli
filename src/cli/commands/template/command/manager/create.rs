use std::fs;

use crate::{
    cli::commands::template::{command::manager::TemplateManager, TemplateError},
    utils::fs::list_subfolders,
};

impl TemplateManager {
    pub fn create_template(
        &self,
        template: &str,
        variant: &Option<String>,
    ) -> Result<(), TemplateError> {
        let template_path = self.templates_path.join(template);

        match variant {
            Some(variant_name) => {
                if !self.templates.contains(&template.to_string()) {
                    return Err(TemplateError::CreatingVariantWithoutDefault);
                }

                let current_variants = list_subfolders(&template_path)
                    .map_err(TemplateError::FailedToReadTemplateDir)?;

                if current_variants.contains(variant_name) {
                    return Err(TemplateError::VariantAlreadyExists);
                }

                let variant_path = template_path.join(variant_name);
                fs::create_dir_all(&variant_path).map_err(|e| TemplateError::Io {
                    context: format!("Failed to create variant folder '{:?}'", variant_path),
                    source: e,
                })?;

                log::info!(
                    "Created variant '{}' for template '{}'",
                    variant_name,
                    template
                );
            }
            None => {
                if self.templates.contains(&template.to_string()) {
                    return Err(TemplateError::TemplateAlreadyExists);
                }

                fs::create_dir_all(&template_path).map_err(|e| TemplateError::Io {
                    context: format!("Failed to create template folder '{:?}'", template_path),
                    source: e,
                })?;

                log::info!("Created new template '{}'", template);
            }
        }

        Ok(())
    }
}
