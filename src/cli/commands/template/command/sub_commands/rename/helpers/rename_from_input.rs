use crate::cli::commands::template::{
    command::sub_commands::rename::RenameSubcommand, service::TemplateService, TemplateError,
};

impl RenameSubcommand {
    pub fn rename_from_input(
        &self,
        service: &TemplateService,
        raw_template: &str,
        raw_variant: Option<&str>,
    ) -> Result<(), TemplateError> {
        let template = service.new_template(raw_template)?;

        match raw_variant {
            Some(v) => {
                let variant = service.new_variant(&template, &v)?;
                let input = service.text("Rename to:")?;
                service.rename_variant(&variant, &input)?;
            }
            None => {
                let input = service.text("Rename to:")?;
                service.rename_template(&template, &input)?;
            }
        };

        Ok(())
    }
}
