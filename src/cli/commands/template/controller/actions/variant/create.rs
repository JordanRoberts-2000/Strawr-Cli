use crate::{
    services::{cli::traits::HasEditorLauncherService, editor_launcher::Editor},
    template::{models::Variant, TemplateController, TemplateError},
};

impl<'c> TemplateController<'c> {
    pub fn create_variant(&self, variant: &Variant, editor: &Editor) -> Result<(), TemplateError> {
        self.service.ensure_template_exists(&variant.template)?;
        self.service.ensure_variant_does_not_exist(variant)?;

        self.service.create_variant(&variant)?;
        self.view.variant_created(&variant);
        self.service.launch_editor(&editor, &variant.path)?;
        Ok(())
    }
}
