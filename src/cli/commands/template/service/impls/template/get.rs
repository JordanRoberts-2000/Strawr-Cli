use crate::{
    template::{service::TemplateService, TemplateError},
    utils,
};

impl<'svc> TemplateService<'svc> {
    pub fn get_templates(&self) -> Result<Vec<String>, TemplateError> {
        let templates = utils::fs::sub_dirs(&self.templates_path)?;
        Ok(templates)
    }
}
