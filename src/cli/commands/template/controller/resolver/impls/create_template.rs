use crate::{
    services::editor_launcher::Editor,
    template::{controller::resolver::TemplateResolver, TemplateError},
};

impl<'c> TemplateResolver<'c> {
    pub fn create_template(self, editor: &Editor) -> Result<(), TemplateError> {
        if let Some(v) = &self.variant {
            return self.controller.create_variant(v, editor);
        }

        self.controller.create_template(&self.template, editor)
    }
}
