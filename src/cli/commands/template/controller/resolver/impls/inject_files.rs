use std::path::Path;

use crate::template::{controller::resolver::TemplateResolver, TemplateError};

impl<'c> TemplateResolver<'c> {
    pub fn inject_files(self, output: &Path) -> Result<(), TemplateError> {
        if let Some(v) = &self.variant {
            return self.controller.inject_variant_files(v, output);
        }

        return self
            .controller
            .inject_template_files(&self.template, output);
    }
}
