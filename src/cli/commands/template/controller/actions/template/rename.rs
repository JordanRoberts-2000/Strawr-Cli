use crate::template::{models::Template, TemplateController, TemplateError};

impl<'c> TemplateController<'c> {
    pub fn rename_template(&self, template: &Template) -> Result<(), TemplateError> {
        self.service.ensure_template_exists(&template)?;

        let new_template = self.prompt_template_name()?;
        self.service.ensure_template_does_not_exist(&new_template)?;

        self.service.rename_template(&template, &new_template)?;
        self.view.template_renamed(&template, &new_template);
        Ok(())
    }
}
