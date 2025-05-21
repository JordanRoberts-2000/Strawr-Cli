use crate::{
    template::{service::TemplateService, TemplateError},
    utils,
};

impl<'svc> TemplateService<'svc> {
    pub fn init_templates_folder(&self) -> Result<(), TemplateError> {
        if !self.templates_path.is_dir() {
            utils::fs::create_dir_all(&self.templates_path)?;
        }

        Ok(())
    }
}
