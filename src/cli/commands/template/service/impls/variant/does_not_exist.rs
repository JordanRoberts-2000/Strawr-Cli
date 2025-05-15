use crate::template::{models::Variant, service::TemplateService, TemplateError};

impl TemplateService {
    pub fn ensure_variant_does_not_exist(&self, variant: &Variant) -> Result<(), TemplateError> {
        if variant.path.exists() {
            return Err(TemplateError::VariantAlreadyExists {
                variant: variant.id.to_string(),
                template: variant.get_template_id().to_string(),
            });
        }

        Ok(())
    }
}
