use crate::{
    cli::commands::template::{
        service::TemplateService,
        utils::{Template, Variant},
        TemplateError,
    },
    error::IoError,
    utils::fs::subfolders,
};

impl<'a> TemplateService<'a> {
    pub fn select_all(&self, template: &Template, msg: &str) -> Result<Variant, TemplateError> {
        let variants =
            subfolders(&template.path).map_err(|e| IoError::ReadDir(e, template.path.clone()))?;

        if variants.is_empty() {
            return Err(TemplateError::NoVariants(template.name.clone()))?;
        }

        let input = self.select(&variants, msg)?;
        log::debug!("User selected variant: '{}'", input);

        let variant_path = template.path.join(&input);
        Ok(Variant::new(&input, &variant_path))
    }
}
