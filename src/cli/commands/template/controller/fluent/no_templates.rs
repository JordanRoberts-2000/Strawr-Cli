use std::marker::PhantomData;

use crate::template::{controller::enums::NoTemplates, TemplateError};

use super::core::{NoTemplatesHandled, Start, TemplateFlow};

impl<'c> TemplateFlow<'c, Start> {
    pub fn if_no_templates(
        &self,
        option: NoTemplates,
    ) -> Result<TemplateFlow<'c, NoTemplatesHandled>, TemplateError> {
        if self.ctrl.service.has_templates()? {
            return Ok(TemplateFlow {
                _state: PhantomData,
                ctrl: self.ctrl,
                handled_no_templates: false,
                template: None,
                variant: None,
                editor: self.editor.clone(),
            });
        }

        match option {
            NoTemplates::Msg(msg) => self.ctrl.view.msg(&msg),
            NoTemplates::PromptCreation => {
                let template = self.ctrl.prompt_template_name()?;
                self.ctrl.create_template(&template, &self.editor)?;
            }
        };

        Ok(TemplateFlow {
            _state: PhantomData,
            ctrl: self.ctrl,
            handled_no_templates: true,
            template: None,
            variant: None,
            editor: self.editor.clone(),
        })
    }
}
