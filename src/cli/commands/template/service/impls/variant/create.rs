use crate::{
    error::IoError,
    template::{models::Variant, service::TemplateService, TemplateError},
};

impl<'svc> TemplateService<'svc> {
    pub fn create_variant(&self, variant: &Variant) -> Result<(), TemplateError> {
        std::fs::create_dir_all(&variant.path)
            .map_err(|e| IoError::CreateDir(e, variant.path.to_path_buf()))?;
        Ok(())
    }
}
