use crate::cli::commands::template::{service::TemplateService, TemplateError};

impl<'a> TemplateService<'a> {
    pub fn text(&self, msg: &str) -> Result<String, TemplateError> {
        let input = self.input.text(msg)?;

        Ok(input)
    }
}
