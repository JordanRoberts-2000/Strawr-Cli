use crate::template::{TemplateError, TemplateService};

impl<'a> TemplateService<'a> {
    pub fn confirm(&self, msg: &str) -> Result<bool, TemplateError> {
        let confirmed = self.input.confirm(msg)?;

        Ok(confirmed)
    }
}
