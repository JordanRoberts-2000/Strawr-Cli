use crate::template::{models::Template, service::TemplateService, TemplateError};

impl TemplateService {
    pub fn delete_template(&self, template: &Template) -> Result<(), TemplateError> {
        self.ensure_template_exists(template)?;

        self.fs.delete_dir_all(&template.path)?;
        Ok(())
    }
}
