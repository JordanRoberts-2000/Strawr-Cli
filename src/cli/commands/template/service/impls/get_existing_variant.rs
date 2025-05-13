use crate::template::{
    models::{Template, Variant},
    service::TemplateService,
    TemplateError,
};

impl TemplateService {
    pub fn get_existing_variant(
        &self,
        template: &Template,
        str: &str,
    ) -> Result<Variant, TemplateError> {
        let valid_name = Variant::validate_name(str)?;
        let variant = Variant::new(&template, &valid_name);

        if !variant.path.exists() {
            return Err(TemplateError::VariantNotFound {
                variant: variant.name.to_string(),
                template: template.name.to_string(),
            });
        }

        Ok(variant)
    }
}
