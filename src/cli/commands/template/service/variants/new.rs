use crate::template::{
    constants::DEFAULT_FOLDER,
    utils::{Template, Variant},
    TemplateError, TemplateService,
};

impl<'a> TemplateService<'a> {
    pub fn new_variant(
        &self,
        template: &Template,
        variant: &str,
    ) -> Result<Variant, TemplateError> {
        let variant_path = template.path.join(&variant);

        if !variant_path.is_dir() {
            return Err(TemplateError::VariantNotFound {
                template: template.name.to_owned(),
                variant: variant.to_owned(),
            });
        }

        if variant == DEFAULT_FOLDER {
            return Err(TemplateError::Validation(format!(
                "'{DEFAULT_FOLDER}' can't be renamed"
            )));
        }

        Ok(Variant::new(&template, variant, &variant_path))
    }
}
