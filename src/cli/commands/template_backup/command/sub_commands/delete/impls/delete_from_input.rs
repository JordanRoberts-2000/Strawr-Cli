use crate::template::{
    manager::TemplateManager, sub_commands::delete::DeleteSubcommand, TemplateError,
};

impl DeleteSubcommand {
    pub fn delete_from_input(
        &self,
        manager: &TemplateManager,
        raw_template: &str,
        raw_variant: Option<&str>,
    ) -> Result<(), TemplateError> {
        let template = manager.new_template(raw_template)?;

        match raw_variant {
            Some(v) => {
                let variant = manager.new_variant(&template, &v)?;
                manager.delete_variant(&variant)?
            }
            None => manager.delete_template(&template)?,
        }

        Ok(())
    }
}
