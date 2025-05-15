use colored::*;
use std::cell::OnceCell;

use crate::prompt::{traits::CliInput, UserInput};

use super::{
    models::{Template, Variant},
    types::{ValidTemplateName, ValidVariantName},
    TemplateError,
};

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
        println!("Template '{}' created successfully", template.id);
    }

    pub fn variant_created(&self, variant: &Variant) {
        if self.muted {
            return;
        }
        println!(
            "Variant '{}' for template '{}' created successfully",
            variant.id,
            variant.get_template_id()
        );
    }

    pub fn template_deleted(&self, template: &Template) {
        if self.muted {
            return;
        }
        println!("Template '{}' deleted successfully", template.id);
    }
    pub fn variant_deleted(&self, variant: &Variant) {
        if self.muted {
            return;
        }
        println!(
            "Variant '{}' from template '{}' deleted successfully",
            variant.id,
            variant.get_template_id()
        );
    }

    pub fn delete_template_confirmation(&self, template: &Template) -> Result<bool, TemplateError> {
        let msg = format!("Are you sure you want to delete template '{}'", template.id);

        let input = self.prompt().confirm(&msg)?;
        Ok(input)
    }

    pub fn delete_variant_confirmation(&self, variant: &Variant) -> Result<bool, TemplateError> {
        let msg = format!(
            "Are you sure you want to delete variant '{}' from template '{}'",
            variant.id,
            variant.get_template_id()
        );

        let input = self.prompt().confirm(&msg)?;
        Ok(input)
    }

    pub fn no_templates(&self) -> Result<bool, TemplateError> {
        let msg = "No templates currently exist, would you like to create one?";

        let input = self.prompt().confirm(msg)?;
        Ok(input)
    }

    pub fn enter_template_name(&self) -> Result<String, TemplateError> {
        let msg = "Enter template name:";
        let input = self.prompt().text(msg)?;

        Ok(input)
    }

    pub fn enter_variant_name(&self) -> Result<String, TemplateError> {
        let msg = "Enter variant name:";
        let input = self.prompt().text(msg)?;

        Ok(input)
    }

    pub fn select_template(
        &self,
        options: &Vec<String>,
    ) -> Result<ValidTemplateName, TemplateError> {
        let msg = "Select template:";
        let input = self.prompt().search(options, msg)?;

        Ok(ValidTemplateName::new(&input))
    }

    pub fn select_variant(&self, options: &Vec<String>) -> Result<ValidVariantName, TemplateError> {
        let msg = "Select variant:";
        let input = self.prompt().search(options, msg)?;

        Ok(ValidVariantName::new(&input))
    }

    pub fn output_not_empty_warning(&self) -> Result<bool, TemplateError> {
        let msg = "The output directory is not empty. Do you still want to inject template files?";
        let input = self.prompt().confirm(msg)?;

        Ok(input)
    }

    pub fn warn_variant_ignored(&self) {
        let msg =
            format!("⚠️  You provided both inline and --variant; only inline was used.").yellow();
        println!("{msg}");
    }

    pub fn template_renamed(&self, template: &Template, new_name: &ValidTemplateName) {
        if self.muted {
            return;
        }
        println!(
            "Successfully renamed template '{}' to '{}' successfully",
            template.id, new_name
        )
    }

    pub fn variant_renamed(&self, variant: &Variant, new_name: &ValidVariantName) {
        if self.muted {
            return;
        }
        println!(
            "Successfully renamed variant '{}' to '{}'",
            variant.id, new_name
        )
    }
}
