use crate::template::{
    controller::enums::TemplateOrVariant,
    models::{markers::Unchecked, Template, Variant},
    TemplateError,
};

use super::super::core::{NoTemplatesHandled, TemplateFlow};

impl<'c> TemplateFlow<'c, NoTemplatesHandled> {
    pub fn create_template(&self) -> Result<(), TemplateError> {
        if self.handled_no_templates {
            return Ok(());
        }

        let choice = self.ctrl.view.select_template_or_variant()?;

        match choice {
            TemplateOrVariant::Template => {
                let template = self.ctrl.prompt_template_name()?;
                self.ctrl.create_template(&template, &self.editor)?;
            }
            TemplateOrVariant::Variant => {
                let template = self.ctrl.select_template()?;
                let variant = self.ctrl.prompt_variant_name(&template)?;
                self.ctrl.create_variant(&variant, &self.editor)?;
            }
        };

        Ok(())
    }
}
