use crate::cli::commands::template::{
    command::sub_commands::delete::DeleteSubcommand, service::TemplateService, TemplateError,
};

impl DeleteSubcommand {
    pub fn delete_from_input(
        &self,
        service: &TemplateService,
        raw_template: &str,
        raw_variant: Option<&str>,
    ) -> Result<(), TemplateError> {
        let template = service.new_template(raw_template)?;

        match raw_variant {
            Some(v) => {
                let variant = service.new_variant(&template, &v)?;
                service.delete_variant(&variant)?
            }
            None => service.delete_template(&template)?,
        }

        Ok(())
    }
}
