use crate::template::{models::Variant, service::TemplateService, TemplateError};

impl<'svc> TemplateService<'svc> {
    pub fn create_variant(&self, variant: &Variant) -> Result<(), TemplateError> {
        self.fs.create_dir_all(&variant.path)?;
        Ok(())
    }
}
