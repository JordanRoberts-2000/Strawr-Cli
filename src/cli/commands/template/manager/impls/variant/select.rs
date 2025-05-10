use crate::{
    error::IoError,
    template::{
        constants::DEFAULT_FOLDER,
        utils::{Template, Variant},
        TemplateError, TemplateManager,
    },
    utils::fs::subfolders,
};

impl<'a> TemplateManager<'a> {
    pub fn select_variant(&self, template: &Template, msg: &str) -> Result<Variant, TemplateError> {
        let mut variants =
            subfolders(&template.path).map_err(|e| IoError::ReadDir(e, template.path.clone()))?;

        variants.retain(|s| s != DEFAULT_FOLDER);

        if variants.is_empty() {
            return Err(TemplateError::NoVariants(template.name.clone()))?;
        }

        let input = self.service.prompt.search(&variants, msg)?;
        log::debug!("User selected variant: '{}'", input);

        let variant_path = template.path.join(&input);
        Ok(Variant::new(&template, &input, &variant_path))
    }
}
