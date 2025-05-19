use crate::{
    services::editor_launcher::Editor,
    template::{
        models::{Template, Variant},
        TemplateController, TemplateError,
    },
};

pub struct NoInputBuilder<'c> {
    template: Option<Template>,
    variant: Option<Variant>,
    done: bool,
    controller: &'c TemplateController<'c>,
}

impl<'c> NoInputBuilder<'c> {
    pub fn new(controller: &'c TemplateController) -> Self {
        Self {
            template: None,
            variant: None,
            done: false,
            controller,
        }
    }

    pub fn select(&mut self, select_type: Selection) -> Result<&Self, TemplateError> {
        if self.done || !self.controller.service.has_templates()? {
            return Ok(self);
        }

        match select_type {
            Selection::TemplateOrVariant => {
                let template = self.controller.select_template()?;

                if self.controller.service.has_variants(&template)? {
                    // to prompt
                }
            }
            Selection::VariantOrDefault => {
                // let template = self.controller.select_template()?;
                // self.template = Some(template);

                // if self.controller.service.has_variants(&template)? {
                //     let variant = self
                //         .controller
                //         .select_variant_including_default(&template)?;
                //     if variant.id != "default" {
                //         self.variant = Some(variant);
                //     }
                // }
            }
        }

        Ok(self)
    }
}

pub enum Selection {
    TemplateOrVariant,
    VariantOrDefault,
}
