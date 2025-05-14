use crate::template::{models::Template, types::ValidTemplateName, TemplateError, TemplateService};

impl TemplateService {
    pub fn rename_template(
        &self,
        template: &Template,
        new_name: &ValidTemplateName,
    ) -> Result<(), TemplateError> {
        let new_template_path = self.templates_path.join(new_name.as_str());
        self.fs.rename(&template.path, &new_template_path)?;

        Ok(())
    }
}
