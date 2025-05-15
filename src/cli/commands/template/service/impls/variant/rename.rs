use crate::template::{models::Variant, types::ValidVariantName, TemplateError, TemplateService};

impl TemplateService {
    pub fn rename_variant(
        &self,
        variant: &Variant,
        new_name: &ValidVariantName,
    ) -> Result<(), TemplateError> {
        let new_variant = Variant::new(&variant.template, new_name);

        self.ensure_variant_exists(variant)?;
        self.ensure_variant_does_not_exist(&new_variant)?;

        self.fs.rename(&variant.path, &new_variant.path)?;
        Ok(())
    }
}
