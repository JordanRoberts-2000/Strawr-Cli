use crate::template::{
    models::{markers::Unchecked, Template, Variant},
    TemplateError,
};

use super::super::core::{Selected, TemplateFlow};

impl<'c> TemplateFlow<'c, Selected> {
    pub fn edit_template(&self) -> Result<(), TemplateError> {
        if self.handled_no_templates {
            return Ok(());
        }

        if let Some(v) = &self.variant {
            return self.ctrl.service.open_variant_in_editor(v, &self.editor);
        }

        if let Some(t) = &self.template {
            return self.ctrl.service.open_template_in_editor(t, &self.editor);
        }

        Ok(())
    }
}
