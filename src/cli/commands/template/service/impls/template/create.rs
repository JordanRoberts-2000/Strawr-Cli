use crate::template::{models::Template, service::TemplateService, TemplateError};

impl TemplateService {
    pub fn create_template(&self, template: &Template) -> Result<(), TemplateError> {
        self.ensure_template_does_not_exist(template)?;

        self.fs.create_dir_all(&template.default_path())?;
        Ok(())
    }
}
