use crate::{
    services::editor_launcher::Editor,
    template::{controller::resolver::TemplateResolver, TemplateError},
};

impl<'c> TemplateResolver<'c> {
    pub fn edit_template(self, editor: &Editor) -> Result<(), TemplateError> {
        if let Some(v) = self.variant {
            let variant = v.ensure_exists()?;
            self.controller
                .service
                .open_variant_in_editor(&variant, editor)?;
        } else {
            let t = &self.template.ensure_exists()?;
            let variant = self.controller.select_variant_including_default(t)?;

            self.controller
                .service
                .open_variant_in_editor(&variant, editor)?;
        }

        Ok(())
    }
}
