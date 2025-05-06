use crate::cli::commands::template::{
    command::{
        service::TemplateService,
        utils::{Template, Variant},
    },
    TemplateError,
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

        Ok(Variant::new(variant, &variant_path))
    }
}
