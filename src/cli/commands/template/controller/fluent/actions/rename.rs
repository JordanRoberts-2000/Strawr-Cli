use crate::template::{
    models::{markers::Unchecked, Template, Variant},
    TemplateError,
};

use super::super::core::{Selected, TemplateFlow};

impl<'c> TemplateFlow<'c, Selected> {
    pub fn rename_template(&self) -> Result<(), TemplateError> {
        if self.handled_no_templates {
            return Ok(());
        }

        if let Some(v) = &self.variant {
            let variant = Variant::<Unchecked>::from(v);
            return self.ctrl.rename_variant(&variant);
        }

        if let Some(t) = &self.template {
            let template = Template::<Unchecked>::from(t);
            return self.ctrl.rename_template(&template);
        }

        Ok(())
    }
}
