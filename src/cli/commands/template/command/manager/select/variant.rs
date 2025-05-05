use crate::{
    cli::commands::template::{
        command::{manager::TemplateManager, utils::Template},
        TemplateError, DEFAULT_FOLDER,
    },
    error::IoError,
    utils::fs::subfolders,
};

impl<'a> TemplateManager<'a> {
    pub fn select_variant(&self, template: &Template, msg: &str) -> Result<String, TemplateError> {
        let mut variants =
            subfolders(&template.path).map_err(|e| IoError::ReadDir(e, template.path.clone()))?;

        variants.retain(|s| s != DEFAULT_FOLDER);

        if variants.is_empty() {
            Err(TemplateError::NoVariants(template.name.clone()))
        } else {
            let selected_variant = self.ctx.input.select(&variants, msg)?;
            log::debug!("User selected variant: '{}'", selected_variant);

            Ok(selected_variant)
        }
    }
}
