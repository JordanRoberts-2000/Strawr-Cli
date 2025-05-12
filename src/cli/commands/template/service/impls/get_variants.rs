use crate::template::{models::Template, service::TemplateService, TemplateError};

impl TemplateService {
    pub fn get_variants(&self, template: &Template) -> Result<Vec<String>, TemplateError> {
        let variants = self.fs.sub_dirs(&template.path)?;
        Ok(variants)
    }
    // pub fn has_variants(&self, template: &Template) -> Result<bool, TemplateError> {
    //     let has_variants = self.fs.min_entries(&template.path, 2)?;
    //     Ok(has_variants)
    // }
}
