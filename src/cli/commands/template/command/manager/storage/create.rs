use crate::cli::commands::template::{command::manager::TemplateManager, TemplateError};

impl TemplateManager {
    pub fn create_template(
        &self,
        template: &String,
        variant: &Option<String>,
    ) -> Result<(), TemplateError> {
        Ok(())
    }
}
