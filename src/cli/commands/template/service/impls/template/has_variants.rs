use crate::{
    template::{
        models::{markers::Exists, Template},
        service::TemplateService,
        TemplateError,
    },
    utils,
};

impl<'svc> TemplateService<'svc> {
    pub(crate) fn has_variants(&self, template: &Template<Exists>) -> Result<bool, TemplateError> {
        let has_variants = utils::fs::dir_entry_count(&template.default_path())? > 1;
        Ok(has_variants)
    }
}
