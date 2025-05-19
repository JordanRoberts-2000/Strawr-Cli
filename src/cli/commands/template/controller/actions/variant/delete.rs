use crate::template::{models::Variant, TemplateController, TemplateError};

impl<'c> TemplateController<'c> {
    pub fn delete_variant(&self, variant: &Variant) -> Result<(), TemplateError> {
        self.service.ensure_template_exists(&variant.template)?;
        self.service.ensure_variant_does_not_exist(variant)?;

        if !self.view.delete_variant_confirmation(&variant)? {
            return Ok(());
        }

        self.service.delete_variant(&variant)?;
        self.view.variant_deleted(&variant);
        Ok(())
    }
}
