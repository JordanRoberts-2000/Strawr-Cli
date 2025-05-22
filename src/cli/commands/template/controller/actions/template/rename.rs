use crate::template::{
    models::{
        markers::{DoesNotExist, Exists},
        Template,
    },
    TemplateController, TemplateError,
};

impl<'c> TemplateController<'c> {
    pub fn rename_template(&self, template: &Template) -> Result<(), TemplateError> {
        let template: Template<Exists> = template.ensure_exists()?;

        let new_template: Template<DoesNotExist> =
            self.prompt_template_name()?.ensure_does_not_exist()?;

        self.service.rename_template(&template, &new_template)?;
        self.view.template_renamed(&template, &new_template);
        Ok(())
    }
}
