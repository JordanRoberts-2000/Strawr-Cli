use crate::{
    services::{cli::traits::HasEditorLauncherService, editor_launcher::Editor},
    template::{
        models::{markers::Exists, Variant},
        service::TemplateService,
        TemplateError,
    },
};

impl<'svc> TemplateService<'svc> {
    pub(crate) fn open_variant_in_editor(
        &self,
        variant: &Variant<Exists>,
        editor: &Editor,
    ) -> Result<(), TemplateError> {
        self.launch_editor(editor, variant.path())?;
        Ok(())
    }
}
