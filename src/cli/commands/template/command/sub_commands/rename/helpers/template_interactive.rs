use crate::cli::commands::template::{
    command::sub_commands::rename::RenameSubcommand, service::TemplateService, TemplateError,
};

impl RenameSubcommand {
    pub fn rename_template_interactive(
        &self,
        service: &TemplateService,
    ) -> Result<(), TemplateError> {
        let template = service.select_template("Template to edit:")?;
        let input = service.text("Rename to:")?;

        service.rename_template(&template, &input)?;
        Ok(())
    }
}
