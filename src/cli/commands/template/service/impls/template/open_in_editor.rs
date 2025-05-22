use crate::{
    services::{cli::traits::HasEditorLauncherService, editor_launcher::Editor},
    template::{
        models::{markers::Exists, Template},
        service::TemplateService,
        TemplateError,
    },
};

impl<'svc> TemplateService<'svc> {
    pub(crate) fn open_template_in_editor(
        &self,
        template: &Template<Exists>,
        editor: &Editor,
    ) -> Result<(), TemplateError> {
        self.launch_editor(editor, template.default_path())?;
        Ok(())
    }
}
