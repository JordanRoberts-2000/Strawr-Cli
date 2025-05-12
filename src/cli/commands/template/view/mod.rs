use std::cell::OnceCell;

use crate::prompt::{traits::CliInput, UserInput};

use super::{models::Template, TemplateError};

pub struct TemplateView {
    prompt: OnceCell<Box<dyn CliInput>>,
    muted: bool,
}

impl TemplateView {
    pub fn new(muted: &bool) -> Self {
        Self {
            prompt: OnceCell::new(),
            muted: *muted,
        }
    }

    pub fn prompt(&self) -> &dyn CliInput {
        self.prompt.get_or_init(|| Box::new(UserInput)).as_ref()
    }

    pub fn template_created(&self, template: &Template) {
        if self.muted {
            return;
        }
        println!("Template '{}' created successfully", template.name);
    }

    pub fn template_deleted(&self, template: &Template) {
        if self.muted {
            return;
        }
        println!("Template '{}' deleted successfully", template.name);
    }

    pub fn delete_confirmation(&self, template: &Template) -> Result<bool, TemplateError> {
        let msg = format!(
            "Are you sure you want to delete template '{}'",
            template.name
        );

        let input = self.prompt().confirm(&msg)?;
        Ok(input)
    }

    pub fn no_templates(&self) -> Result<bool, TemplateError> {
        let msg = "No templates currently exist, would you like to create one?";

        let input = self.prompt().confirm(msg)?;
        Ok(input)
    }

    pub fn enter_template_name(&self) -> Result<bool, TemplateError> {
        let msg = "Enter template name:";
        let input = self.prompt().text(msg)?;

        Ok(input)
    }

    pub fn select_template(&self, options: &Vec<String>) -> Result<String, TemplateError> {
        let msg = "Select template:";
        let input = self.prompt().search(options, msg)?;

        Ok(input)
    }

    pub fn select_variant(&self, options: &Vec<String>) -> Result<String, TemplateError> {
        let msg = "Select variant:";
        let input = self.prompt().search(options, msg)?;

        Ok(input)
    }

    pub fn output_not_empty_warning(&self) -> Result<bool, TemplateError> {
        let msg = "The output directory is not empty. Do you still want to inject template files?";
        let input = self.prompt().confirm(msg)?;

        Ok(input)
    }
}
