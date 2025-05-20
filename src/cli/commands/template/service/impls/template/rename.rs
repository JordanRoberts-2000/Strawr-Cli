use crate::{
    error::IoError,
    template::{models::Template, TemplateError, TemplateService},
};

impl<'svc> TemplateService<'svc> {
    pub fn rename_template(
        &self,
        template: &Template,
        new_template: &Template,
    ) -> Result<(), TemplateError> {
        std::fs::rename(&template.path, &new_template.path).map_err(|e| {
            IoError::Rename(
                e,
                template.path.to_path_buf(),
                new_template.path.to_path_buf(),
            )
        })?;
        Ok(())
    }
}
