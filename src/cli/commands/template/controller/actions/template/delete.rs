use crate::template::{
    models::{markers::Exists, Template},
    TemplateController, TemplateError,
};

impl<'c> TemplateController<'c> {
    pub fn delete_template(&self, template: &Template) -> Result<(), TemplateError> {
        let template: Template<Exists> = template.ensure_exists()?;

        if !self.view.delete_template_confirmation(&template)? {
            return Ok(());
        }

        self.service.delete_template(&template)?;
        self.view.template_deleted(&template);

        Ok(())
    }
}
