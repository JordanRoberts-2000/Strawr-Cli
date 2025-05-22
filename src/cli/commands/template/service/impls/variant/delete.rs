use crate::{
    template::{
        models::{markers::Exists, Variant},
        TemplateError, TemplateService,
    },
    utils,
};

impl<'svc> TemplateService<'svc> {
    pub fn delete_variant(&self, variant: &Variant<Exists>) -> Result<(), TemplateError> {
        utils::fs::trash(&variant.path())?;
        Ok(())
    }
}
