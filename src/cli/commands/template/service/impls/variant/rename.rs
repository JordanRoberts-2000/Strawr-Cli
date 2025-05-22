use crate::{
    template::{
        models::{
            markers::{DoesNotExist, Exists},
            Variant,
        },
        TemplateError, TemplateService,
    },
    utils,
};

impl<'svc> TemplateService<'svc> {
    pub fn rename_variant(
        &self,
        variant: &Variant<Exists>,
        new_variant: &Variant<DoesNotExist>,
    ) -> Result<(), TemplateError> {
        utils::fs::rename(&variant.path(), &new_variant.path())?;
        Ok(())
    }
}
