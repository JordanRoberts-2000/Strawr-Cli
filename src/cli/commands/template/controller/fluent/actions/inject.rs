use std::path::Path;

use crate::template::{
    models::{markers::Unchecked, Template, Variant},
    TemplateError,
};

use super::super::core::{Selected, TemplateFlow};

impl<'c> TemplateFlow<'c, Selected> {
    pub fn inject_files(&self, output: &Path) -> Result<(), TemplateError> {
        if self.handled_no_templates {
            return Ok(());
        }

        if let Some(v) = &self.variant {
            let variant = Variant::<Unchecked>::from(v);
            return self.ctrl.inject_variant_files(&variant, output);
        }

        if let Some(t) = &self.template {
            let template = Template::<Unchecked>::from(t);
            return self.ctrl.inject_template_files(&template, output);
        }

        Ok(())
    }
}
