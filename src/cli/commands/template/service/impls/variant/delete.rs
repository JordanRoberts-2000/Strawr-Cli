use crate::template::{models::Variant, TemplateError, TemplateService};

impl TemplateService {
    pub fn delete_variant(&self, variant: &Variant) -> Result<(), TemplateError> {
        self.ensure_variant_exists(variant)?;

        self.fs.delete_dir_all(&variant.path)?;
        Ok(())
    }
}
