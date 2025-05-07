use crate::cli::commands::template::{service::TemplateService, utils::Template, TemplateError};

impl<'a> TemplateService<'a> {
    pub fn new_template(&self, template: &str) -> Result<Template, TemplateError> {
        let template_path = self.templates_path.join(template);

        if !template_path.is_dir() {
            return Err(TemplateError::TemplateNotFound(template.to_string()));
        }

        Ok(Template::new(template, &template_path))
    }
}
