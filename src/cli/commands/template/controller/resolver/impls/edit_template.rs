use crate::{
    services::{cli::traits::HasEditorLauncherService, editor_launcher::Editor},
    template::{controller::resolver::TemplateResolver, TemplateError},
};

impl<'c> TemplateResolver<'c> {
    pub fn edit_template(self, editor: &Editor) -> Result<(), TemplateError> {
        if let Some(v) = self.variant {
            self.controller.service.launch_editor(editor, &v.path())?;
        } else {
            let variant = self
                .controller
                .select_variant_including_default(&self.template)?;
            self.controller
                .service
                .launch_editor(&editor, &variant.path())?;
        }

        Ok(())
    }
}
