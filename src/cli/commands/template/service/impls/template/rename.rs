use crate::template::{models::Template, TemplateError, TemplateService};

impl TemplateService {
    pub fn rename_template(
        &self,
        template: &Template,
        new_template: &Template,
    ) -> Result<(), TemplateError> {
        self.fs.rename(&template.path, &new_template.path)?;
        Ok(())
    }
}
