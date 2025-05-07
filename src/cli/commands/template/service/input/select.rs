use crate::cli::commands::template::{service::TemplateService, TemplateError};

impl<'a> TemplateService<'a> {
    pub fn select(&self, options: &[String], msg: &str) -> Result<String, TemplateError> {
        let selected = self.input.select(options, msg)?;

        Ok(selected)
    }
}
