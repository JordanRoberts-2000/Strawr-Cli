use crate::{
    template::{
        models::{
            markers::{DoesNotExist, Exists},
            Template,
        },
        TemplateError, TemplateService,
    },
    utils,
};

impl<'svc> TemplateService<'svc> {
    pub fn rename_template(
        &self,
        template: &Template<Exists>,
        new_template: &Template<DoesNotExist>,
    ) -> Result<(), TemplateError> {
        utils::fs::rename(&template.path(), &new_template.path())?;
        Ok(())
    }
}
