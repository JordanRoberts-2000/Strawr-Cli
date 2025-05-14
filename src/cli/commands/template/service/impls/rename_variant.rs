use crate::template::{models::Variant, types::ValidVariantName, TemplateError, TemplateService};

impl TemplateService {
    pub fn rename_variant(
        &self,
        variant: &Variant,
        new_name: &ValidVariantName,
    ) -> Result<(), TemplateError> {
        let new_variant_path = variant.template_path.join(new_name.as_str());
        self.fs.rename(&variant.path, &new_variant_path)?;

        Ok(())
    }
}
