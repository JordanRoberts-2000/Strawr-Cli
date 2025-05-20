use crate::template::{models::Variant, TemplateError, TemplateService};

impl<'svc> TemplateService<'svc> {
    pub fn rename_variant(
        &self,
        variant: &Variant,
        new_variant: &Variant,
    ) -> Result<(), TemplateError> {
        self.fs.rename(&variant.path, &new_variant.path)?;
        Ok(())
    }
}
