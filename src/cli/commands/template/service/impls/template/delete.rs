use crate::{
    template::{models::Template, service::TemplateService, TemplateError},
    utils,
};

impl<'svc> TemplateService<'svc> {
    pub fn delete_template(&self, template: &Template) -> Result<(), TemplateError> {
        utils::fs::trash(&template.path)?;
        Ok(())
    }
}
