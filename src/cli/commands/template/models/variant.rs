use std::path::PathBuf;

use crate::template::{
    models::Template,
    types::{ValidTemplateName, ValidVariantName},
};

pub struct Variant {
    pub id: ValidVariantName,
    pub path: PathBuf,
    pub template: Template,
}

impl Variant {
    pub fn new(template: &Template, id: &ValidVariantName) -> Self {
        let path = template.path.join(id.as_str());
        Self {
            id: id.clone(),
            path,
            template: template.clone(),
        }
    }

    pub fn get_template_id(&self) -> ValidTemplateName {
        self.template.id.clone()
    }
}
