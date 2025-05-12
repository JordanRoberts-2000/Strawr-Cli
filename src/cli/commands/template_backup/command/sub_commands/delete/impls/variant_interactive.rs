use crate::template::{sub_commands::delete::DeleteSubcommand, TemplateError, TemplateManager};

impl DeleteSubcommand {
    pub fn delete_variant_interactive(
        &self,
        manager: &TemplateManager,
    ) -> Result<(), TemplateError> {
        let template = manager.select_template("Select a template to see variants:")?;
        let variant = manager.select_variant(&template, "Select variant to delete:")?;
        manager.delete_variant(&variant)
    }
}
