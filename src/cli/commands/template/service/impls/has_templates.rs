use crate::{
    template::{service::TemplateService, TemplateError},
    utils::fs::dir_empty,
};

impl<'svc> TemplateService<'svc> {
    pub fn has_templates(&self) -> Result<bool, TemplateError> {
        let has_templates_no_template = dir_empty(&self.templates_path)?;
        Ok(!has_templates_no_template)
    }
}
