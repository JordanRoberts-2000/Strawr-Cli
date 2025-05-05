use crate::{
    cli::commands::template::{
        command::{manager::TemplateManager, utils::Template},
        TemplateError,
    },
    error::IoError,
    utils::fs::subfolders,
};

impl<'a> TemplateManager<'a> {
    pub fn select_variant_or_default(
        &self,
        template: &Template,
        msg: &str,
    ) -> Result<String, TemplateError> {
        let variants =
            subfolders(&template.path).map_err(|e| IoError::ReadDir(e, template.path.clone()))?;

        if variants.is_empty() {
            Err(TemplateError::NoVariants(template.name.clone()))
        } else {
            let selected_variant = self.ctx.input.select(&variants, msg)?;
            log::debug!("User selected variant: '{}'", selected_variant);

            Ok(selected_variant)
        }
    }
}
