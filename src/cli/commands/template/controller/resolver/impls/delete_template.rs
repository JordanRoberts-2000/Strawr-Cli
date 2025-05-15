use crate::template::{controller::resolver::TemplateResolver, TemplateError};

impl<'c> TemplateResolver<'c> {
    pub fn delete_template(&self) -> Result<(), TemplateError> {
        if let Some(v) = &self.variant {
            return self.controller.delete_variant(v);
        }

        self.controller.delete_template(&self.template)
    }
}
