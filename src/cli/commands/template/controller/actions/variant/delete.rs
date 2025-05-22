use crate::template::{
    models::{markers::Exists, Variant},
    TemplateController, TemplateError,
};

impl<'c> TemplateController<'c> {
    pub fn delete_variant(&self, variant: &Variant) -> Result<(), TemplateError> {
        let variant: Variant<Exists> = variant.ensure_exists()?;

        if !self.view.delete_variant_confirmation(&variant)? {
            return Ok(());
        }

        self.service.delete_variant(&variant)?;
        self.view.variant_deleted(&variant);
        Ok(())
    }
}
