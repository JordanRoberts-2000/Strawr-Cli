use std::marker::PhantomData;

use crate::template::{controller::enums::TemplateSelect, TemplateError};

use super::core::{CanSelect, Selected, TemplateFlow};

impl<'c, S> TemplateFlow<'c, S>
where
    S: CanSelect,
{
    pub fn select(
        self,
        option: TemplateSelect,
    ) -> Result<TemplateFlow<'c, Selected>, TemplateError> {
        if self.handled_no_templates {
            return Ok(TemplateFlow {
                ctrl: self.ctrl,
                handled_no_templates: self.handled_no_templates,
                template: None,
                variant: None,
                editor: self.editor.clone(),
                _state: PhantomData,
            });
        }

        let (tmpl, var) = match option {
            TemplateSelect::TemplateOrVariant => self.ctrl.select_template_or_variant()?,
            TemplateSelect::DefaultOrVariant => self.ctrl.select_default_or_variant()?,
        };

        Ok(TemplateFlow {
            ctrl: self.ctrl,
            handled_no_templates: self.handled_no_templates,
            template: Some(tmpl),
            variant: var,
            editor: self.editor.clone(),
            _state: PhantomData,
        })
    }
}
