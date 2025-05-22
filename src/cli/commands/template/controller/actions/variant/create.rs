use crate::{
    services::editor_launcher::Editor,
    template::{
        models::{
            markers::{DoesNotExist, Exists},
            Variant,
        },
        TemplateController, TemplateError,
    },
};

impl<'c> TemplateController<'c> {
    pub fn create_variant(&self, variant: &Variant, editor: &Editor) -> Result<(), TemplateError> {
        let to_create: Variant<DoesNotExist> = variant.ensure_does_not_exist()?;
        let created: Variant<Exists> = self.service.create_variant(&to_create)?;

        self.view.variant_created(&created);
        self.service.open_variant_in_editor(&created, &editor)?;
        Ok(())
    }
}
