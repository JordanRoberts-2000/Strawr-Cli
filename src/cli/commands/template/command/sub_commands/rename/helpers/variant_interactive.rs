use crate::cli::commands::template::{
    command::sub_commands::rename::RenameSubcommand, service::TemplateService, TemplateError,
};

impl RenameSubcommand {
    pub fn rename_variant_interactive(
        &self,
        service: &TemplateService,
    ) -> Result<(), TemplateError> {
        let template = service.select_template("Select a template to see variants:")?;
        let variant = service.select_variant(&template, "Select variant to edit:")?;
        let input = service.text("Rename to:")?;

        service.rename_variant(&variant, &input)?;
        Ok(())
    }
}
