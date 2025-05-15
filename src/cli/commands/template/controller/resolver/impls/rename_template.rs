use crate::template::{controller::resolver::TemplateResolver, TemplateError};

impl<'c> TemplateResolver<'c> {
    pub fn rename_template(self) -> Result<(), TemplateError> {
        if let Some(v) = self.variant {
            return self.controller.rename_variant(&v);
        }

        self.controller.rename_template(&self.template)
    }
}
