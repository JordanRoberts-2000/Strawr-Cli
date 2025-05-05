use std::path::PathBuf;

use crate::{
    cli::commands::template::{
        command::manager::TemplateManager, TemplateCommand, TemplateError, DEFAULT_FOLDER,
    },
    error::IoError,
    utils::fs::subfolders,
};

impl TemplateCommand {
    pub fn resolve_variant(
        &self,
        manager: &TemplateManager,
        inline_variant: &Option<String>,
        template: &String,
        template_path: &PathBuf,
    ) -> Result<Option<String>, TemplateError> {
        match &self.variant {
            Some(Some(cli_variant)) => {
                if inline_variant.is_some() {
                    log::warn!(
                "':' detected in template input (`{template}`) but --variant was also provided. Using --variant and ignoring inline variant."
            );
                }

                let variant_path = template_path.join(cli_variant);
                if !variant_path.exists() {
                    return Err(TemplateError::VariantNotFound {
                        template: template.clone(),
                        variant: cli_variant.clone(),
                    });
                }

                Ok(Some(cli_variant.clone()))
            }

            Some(None) => {
                let mut subdirs = subfolders(template_path)
                    .map_err(|e| IoError::ReadDir(e, template_path.clone()))?;

                subdirs.retain(|s| s != DEFAULT_FOLDER);

                if subdirs.is_empty() {
                    return Err(TemplateError::NoVariants(template.clone()));
                } else {
                    let selected = manager.ctx.input.select(&subdirs, "Choose variant:")?;
                    Ok(Some(selected))
                }
            }

            None => match inline_variant {
                Some(v) => {
                    let variant_path = template_path.join(v);
                    if !variant_path.exists() {
                        return Err(TemplateError::VariantNotFound {
                            template: template.clone(),
                            variant: v.clone(),
                        });
                    }
                    Ok(Some(v.clone()))
                }
                None => Ok(None),
            },
        }
    }
}
