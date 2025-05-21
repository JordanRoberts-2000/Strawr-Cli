use crate::{
    template::{models::Template, service::TemplateService, TemplateError},
    utils,
};

impl<'svc> TemplateService<'svc> {
    pub fn create_template(&self, template: &Template) -> Result<(), TemplateError> {
        utils::fs::create_dir_all(&template.default_path())?;
        Ok(())
    }
}
