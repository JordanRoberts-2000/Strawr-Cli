use crate::{
    services::cli::traits::HasPromptService,
    template::{
        models::{markers::Exists, Template, Variant},
        TemplateError, TemplateView,
    },
};

impl<'a> TemplateView<'a> {
    pub fn delete_template_confirmation(
        &self,
        template: &Template<Exists>,
    ) -> Result<bool, TemplateError> {
        let msg = format!(
            "Are you sure you want to delete template '{}'",
            template.id()
        );

        let input = self.prompt().confirm(&msg)?;
        Ok(input)
    }

    pub fn delete_variant_confirmation(
        &self,
        variant: &Variant<Exists>,
    ) -> Result<bool, TemplateError> {
        let msg = format!(
            "Are you sure you want to delete variant '{}' from template '{}'",
            variant.id(),
            variant.template_id()
        );

        let input = self.prompt().confirm(&msg)?;
        Ok(input)
    }

    pub fn no_templates_prompt_create_one(&self) -> Result<bool, TemplateError> {
        let msg = "No templates currently exist, would you like to create one?";

        let input = self.prompt().confirm(msg)?;
        Ok(input)
    }
}
