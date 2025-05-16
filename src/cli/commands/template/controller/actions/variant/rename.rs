use crate::template::{models::Variant, TemplateController, TemplateError};

impl TemplateController {
    pub fn rename_variant(&self, variant: &Variant) -> Result<(), TemplateError> {
        self.service.ensure_template_exists(&variant.template)?;
        self.service.ensure_variant_exists(variant)?;

        let new_variant = self.prompt_variant_name(&variant.template)?;
        self.service.ensure_variant_does_not_exist(&new_variant)?;

        self.service.rename_variant(&variant, &new_variant)?;
        self.view.variant_renamed(&variant, &new_variant);
        Ok(())
    }
}
