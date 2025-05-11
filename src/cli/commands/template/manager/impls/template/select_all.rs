use crate::{
    error::IoError,
    template::{
        utils::{Template, Variant},
        TemplateError, TemplateManager,
    },
    utils::fs::subfolders,
};

impl<'a> TemplateManager<'a> {
    pub fn select_all(&self, template: &Template, msg: &str) -> Result<Variant, TemplateError> {
        let variants =
            subfolders(&template.path).map_err(|e| IoError::ReadDir(e, template.path.clone()))?;

        if variants.is_empty() {
            return Err(TemplateError::NoVariants(template.name.clone()))?;
        }

        let input = self.service.prompt().search(&variants, msg)?;
        log::debug!("User selected variant: '{}'", input);

        let variant_path = template.path.join(&input);
        Ok(Variant::new(&template, &input, &variant_path))
    }
}
