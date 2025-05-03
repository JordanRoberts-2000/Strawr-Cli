use crate::cli::commands::template::{
    command::{manager::TemplateManager, TemplateInput},
    TemplateCommand, TemplateError,
};

impl TemplateCommand {
    pub fn handle_template(
        &self,
        manager: &TemplateManager,
        template: &TemplateInput,
    ) -> Result<(), TemplateError> {
        let (template, inline_variant) = template;

        let variant = match &self.variant {
            Some(Some(v)) => {
                if inline_variant.is_some() {
                    log::warn!(
                    "':' detected in template input (`{template}`) but --variant was also provided. Using --variant and ignoring inline variant."
                );
                }
                Some(v.clone())
            }
            Some(None) => {
                //todo Select variant
                Some(String::from("temp"))
            }
            None => inline_variant.clone(),
        };

        manager.inject_template_files(template, &variant)?;
        Ok(())
    }
}
