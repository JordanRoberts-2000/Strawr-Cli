use crate::{
    cli::commands::template::{command::manager::TemplateManager, TemplateError},
    error::io::IoError,
    utils::fs::copy_dir_contents,
};

impl<'a> TemplateManager<'a> {
    pub fn inject_template_files(
        &self,
        template: &str,
        variant: &Option<String>,
    ) -> Result<(), TemplateError> {
        let variant_str = variant.as_deref().unwrap_or("default");
        let source_path = self.templates_path.join(template).join(variant_str);

        if !source_path.exists() {
            return if variant.is_none() {
                Err(TemplateError::TemplateNotFound {
                    template: template.to_string(),
                })
            } else {
                Err(TemplateError::VariantNotFound {
                    template: template.to_string(),
                    variant: variant.clone().unwrap_or_default(),
                })
            };
        }

        let current_dir = std::env::current_dir().map_err(IoError::GetCurrentDir)?;

        copy_dir_contents(&source_path, &current_dir)
            .map_err(|e| IoError::CopyDirContents(e, source_path.clone(), current_dir.clone()))?;

        Ok(())
    }
}
