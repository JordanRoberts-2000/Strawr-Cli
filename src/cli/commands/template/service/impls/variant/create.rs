use crate::{
    template::{
        models::{
            markers::{DoesNotExist, Exists},
            Variant,
        },
        service::TemplateService,
        TemplateError,
    },
    utils,
};

impl<'svc> TemplateService<'svc> {
    pub fn create_variant(
        &self,
        variant: &Variant<DoesNotExist>,
    ) -> Result<Variant<Exists>, TemplateError> {
        utils::fs::create_dir_all(&variant.path())?;
        Ok(Variant::<Exists>::from(variant))
    }
}
