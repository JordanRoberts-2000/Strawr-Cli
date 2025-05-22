use crate::{
    template::{
        models::{markers::Exists, Template},
        service::TemplateService,
        TemplateError,
    },
    utils,
};

impl<'svc> TemplateService<'svc> {
    pub fn delete_template(&self, template: &Template<Exists>) -> Result<(), TemplateError> {
        utils::fs::trash(&template.path())?;
        Ok(())
    }
}
