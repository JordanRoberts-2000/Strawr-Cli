use crate::{
    error::IoError,
    template::{models::Variant, TemplateError, TemplateService},
};

impl<'svc> TemplateService<'svc> {
    pub fn rename_variant(
        &self,
        variant: &Variant,
        new_variant: &Variant,
    ) -> Result<(), TemplateError> {
        std::fs::rename(&variant.path, &new_variant.path).map_err(|e| {
            IoError::Rename(
                e,
                variant.path.to_path_buf(),
                new_variant.path.to_path_buf(),
            )
        })?;
        Ok(())
    }
}
