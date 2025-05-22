use crate::{
    template::{
        models::{markers::Exists, Template},
        service::TemplateService,
        TemplateError,
    },
    utils,
};

impl<'svc> TemplateService<'svc> {
    pub fn get_variants(&self, template: &Template<Exists>) -> Result<Vec<String>, TemplateError> {
        let variants = utils::fs::sub_dirs(&template.path())?;
        Ok(variants)
    }
}
