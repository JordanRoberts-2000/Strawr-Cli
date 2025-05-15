use crate::template::{models::Variant, TemplateController, TemplateError};

impl TemplateController {
    pub fn delete_variant(&self, variant: &Variant) -> Result<(), TemplateError> {
        if !self.view.delete_variant_confirmation(&variant)? {
            return Ok(());
        }

        self.service.delete_variant(&variant)?;
        self.view.variant_deleted(&variant);
        Ok(())
    }
}
