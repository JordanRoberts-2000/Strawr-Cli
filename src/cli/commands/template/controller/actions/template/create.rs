use crate::{
    services::{cli::traits::HasEditorLauncherService, editor_launcher::Editor},
    template::{models::Template, TemplateController, TemplateError},
};

impl<'c> TemplateController<'c> {
    pub fn create_template(
        &self,
        template: &Template,
        editor: &Editor,
    ) -> Result<(), TemplateError> {
        self.service.create_template(template)?;
        self.view.template_created(&template);
        self.service
            .launch_editor(&editor, &template.default_path())?;

        Ok(())
    }
}
