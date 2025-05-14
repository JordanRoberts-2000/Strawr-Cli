use crate::template::{
    models::{Template, Variant},
    service::TemplateService,
    types::ValidVariantName,
    TemplateError,
};

impl TemplateService {
    pub fn get_existing_variant(
        &self,
        template: &Template,
        variant: &ValidVariantName,
    ) -> Result<Variant, TemplateError> {
        let variant = Variant::new(&template, &variant);

        if !variant.path.exists() {
            return Err(TemplateError::VariantNotFound {
                variant: variant.name.to_string(),
                template: template.name.to_string(),
            });
        }

        Ok(variant)
    }
}
