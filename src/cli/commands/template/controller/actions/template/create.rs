use crate::{
    services::editor_launcher::Editor,
    template::{
        models::{
            markers::{DoesNotExist, Exists},
            Template,
        },
        TemplateController, TemplateError,
    },
};

impl<'c> TemplateController<'c> {
    pub fn create_template(
        &self,
        template: &Template,
        editor: &Editor,
    ) -> Result<(), TemplateError> {
        let to_create: Template<DoesNotExist> = template.ensure_does_not_exist()?;
        let created: Template<Exists> = self.service.create_template(&to_create)?;

        self.view.template_created(&created);
        self.service.open_template_in_editor(&created, editor)?;
        Ok(())
    }
}
