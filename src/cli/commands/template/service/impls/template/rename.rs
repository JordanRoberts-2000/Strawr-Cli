use crate::template::{models::Template, types::ValidTemplateName, TemplateError, TemplateService};

impl TemplateService {
    pub fn rename_template(
        &self,
        template: &Template,
        new_name: &ValidTemplateName,
    ) -> Result<(), TemplateError> {
        let new_template = Template::new(new_name, &self.templates_path);

        self.ensure_template_exists(&template)?;
        self.ensure_template_does_not_exist(&new_template)?;

        self.fs.rename(&template.path, &new_template.path)?;
        Ok(())
    }
}
