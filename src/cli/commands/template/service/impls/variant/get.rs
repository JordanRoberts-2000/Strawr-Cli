use crate::{
    template::{models::Template, service::TemplateService, TemplateError},
    utils,
};

impl<'svc> TemplateService<'svc> {
    pub fn get_variants(&self, template: &Template) -> Result<Vec<String>, TemplateError> {
        let variants = utils::fs::sub_dirs(&template.path)?;
        Ok(variants)
    }
}
