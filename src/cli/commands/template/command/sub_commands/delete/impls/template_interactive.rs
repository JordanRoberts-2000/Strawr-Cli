use crate::template::{sub_commands::delete::DeleteSubcommand, TemplateError, TemplateService};

impl DeleteSubcommand {
    pub fn delete_template_interactive(
        &self,
        service: &TemplateService,
    ) -> Result<(), TemplateError> {
        let template = service.select_template("Template to delete:")?;
        service.delete_template(&template)
    }
}
