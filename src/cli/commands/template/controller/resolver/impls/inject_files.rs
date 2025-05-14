use std::path::Path;

use crate::template::{controller::resolver::TemplateResolver, TemplateError};

impl<'c> TemplateResolver<'c> {
    pub fn inject_files(self, output: &Path) -> Result<(), TemplateError> {
        let path = self
            .variant
            .map(|v| v.path)
            .unwrap_or_else(|| self.template.path.clone());
        self.controller.inject_template_files(&path, output)
    }
}
