use crate::template::{sub_commands::delete::DeleteSubcommand, TemplateError, TemplateManager};

impl DeleteSubcommand {
    pub fn delete_template_interactive(
        &self,
        manager: &TemplateManager,
    ) -> Result<(), TemplateError> {
        let template = manager.select_template("Template to delete:")?;
        manager.delete_template(&template)
    }
}
