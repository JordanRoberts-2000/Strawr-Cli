use crate::template::{controller::resolver::TemplateResolver, TemplateError};

impl<'c> TemplateResolver<'c> {
    pub fn delete_template(self) -> Result<(), TemplateError> {
        if let Some(v) = self.variant {
            self.controller.delete_variant(&self.template, &v)?;
        } else {
            self.controller.delete_template(&self.template)?;
        }
        Ok(())
    }
}
