use crate::cli::commands::template::{
    command::sub_commands::delete::DeleteSubcommand, service::TemplateService, TemplateError,
};

impl DeleteSubcommand {
    pub fn delete_variant_interactive(
        &self,
        service: &TemplateService,
    ) -> Result<(), TemplateError> {
        let template = service.select_template("Select a template to see variants:")?;
        let variant = service.select_variant(&template, "Select variant to delete:")?;
        service.delete_variant(&variant)
    }
}
