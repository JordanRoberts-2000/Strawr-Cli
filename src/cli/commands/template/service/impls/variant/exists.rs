use crate::template::{models::Variant, service::TemplateService, TemplateError};

impl TemplateService {
    pub fn ensure_variant_exists(&self, variant: &Variant) -> Result<(), TemplateError> {
        self.ensure_template_exists(&variant.template)?;

        if !variant.path.exists() {
            return Err(TemplateError::VariantNotFound {
                variant: variant.id.to_string(),
                template: variant.get_template_id().to_string(),
            });
        }

        Ok(())
    }
}
