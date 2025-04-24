use std::fs;

use crate::{
    cli::commands::template::{command::manager::TemplateManager, TemplateError},
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
            subfolders(&template_path).map_err(TemplateError::FailedToReadTemplateDir)?;

        match variant {
            Some(variant_name) => {
                if !existing_templates.contains(&template.to_string()) {
                    return Err(TemplateError::CreatingVariantWithoutDefault);
                }

                let current_variants =
                    subfolders(&template_path).map_err(TemplateError::FailedToReadTemplateDir)?;

                if current_variants.contains(variant_name) {
                    return Err(TemplateError::VariantAlreadyExists);
                }

                let variant_path = template_path.join(variant_name);

                // fs::create_dir(&variant_path).map_err(|e| TemplateError::Io {
                //     context: format!(
                //         "Failed to create variant folder '{}'",
                //         variant_path.display()
                //     ),
                //     source: e,
                // })?;

                // log::info!(
                //     "Created variant '{}' for template '{}'",
                //     variant_name,
                //     template
                // );

                fs::create_dir(&variant_path)
                    .with_context::<TemplateError>("failed to create..", &variant_path)?;
                log::info!("Created variant '{variant_name}' for template '{template}'");

                self.ctx.config.default_editor.open(&variant_path)?;
            }
            None => {
                if existing_templates.contains(&template.to_string()) {
                    return Err(TemplateError::TemplateAlreadyExists);
                }

                let default_path = template_path.join("default");
                fs::create_dir_all(&default_path).map_err(|e| TemplateError::Io {
                    context: format!(
                        "Failed to create default template folder '{:?}'",
                        default_path
                    ),
                    source: e,
                })?;

                log::info!("Created new template '{}'", template);

                self.ctx.config.default_editor.open(&default_path)?;
            }
        }

        Ok(())
    }
}
