use crate::{
    template::{models::Template, service::TemplateService, TemplateError},
    utils,
};

impl<'svc> TemplateService<'svc> {
    pub(crate) fn has_variants(&self, template: &Template) -> Result<bool, TemplateError> {
        let has_variants = utils::fs::dir_entry_count(&template.path)? > 1;
        Ok(has_variants)
    }
}
