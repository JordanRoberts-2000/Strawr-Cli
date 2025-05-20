use crate::{
    error::IoError,
    template::{models::Template, service::TemplateService, TemplateError},
};

impl<'svc> TemplateService<'svc> {
    pub fn create_template(&self, template: &Template) -> Result<(), TemplateError> {
        std::fs::create_dir_all(&template.default_path())
            .map_err(|e| IoError::CreateDir(e, template.path.to_path_buf()))?;
        Ok(())
    }
}
