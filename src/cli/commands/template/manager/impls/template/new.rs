use crate::template::{utils::Template, TemplateError, TemplateManager};

impl<'a> TemplateManager<'a> {
    pub fn new_template(&self, template: &str) -> Result<Template, TemplateError> {
        let template_path = self.templates_path.join(template);

        if !template_path.is_dir() {
            return Err(TemplateError::TemplateNotFound(template.to_string()));
        }

        Ok(Template::new(template, &template_path))
    }
}
