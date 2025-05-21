use crate::{
    template::{models::Template, TemplateError, TemplateService},
    utils,
};

impl<'svc> TemplateService<'svc> {
    pub fn rename_template(
        &self,
        template: &Template,
        new_template: &Template,
    ) -> Result<(), TemplateError> {
        utils::fs::rename(&template.path, &new_template.path)?;
        Ok(())
    }
}
