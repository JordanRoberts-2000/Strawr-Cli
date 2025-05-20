use crate::template::{models::Template, service::TemplateService, TemplateError};

impl<'svc> TemplateService<'svc> {
    pub fn delete_template(&self, template: &Template) -> Result<(), TemplateError> {
        self.fs.delete_dir_all(&template.path)?;
        Ok(())
    }
}
