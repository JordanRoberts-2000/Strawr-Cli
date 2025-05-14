use crate::template::{models::Template, TemplateController, TemplateError};

impl TemplateController {
    pub fn delete_template(&self, template: &Template) -> Result<(), TemplateError> {
        if !self.view.delete_template_confirmation(&template)? {
            return Ok(());
        }

        self.service.delete_template(&template)?;
        self.view.template_deleted(&template);

        Ok(())
    }
}
