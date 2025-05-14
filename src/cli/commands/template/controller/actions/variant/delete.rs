use crate::template::{
    models::{Template, Variant},
    TemplateController, TemplateError,
};

impl TemplateController {
    pub fn delete_variant(
        &self,
        template: &Template,
        variant: &Variant,
    ) -> Result<(), TemplateError> {
        if !self.view.delete_variant_confirmation(&template, &variant)? {
            return Ok(());
        }

        self.service.delete_variant(&variant)?;
        self.view.variant_deleted(&template, &variant);

        Ok(())
    }
}
