use crate::{
    template::{models::Variant, TemplateError, TemplateService},
    utils,
};

impl<'svc> TemplateService<'svc> {
    pub fn delete_variant(&self, variant: &Variant) -> Result<(), TemplateError> {
        utils::fs::trash(&variant.path)?;
        Ok(())
    }
}
