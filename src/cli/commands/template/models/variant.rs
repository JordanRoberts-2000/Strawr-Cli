use std::path::PathBuf;

use crate::template::{
    models::Template,
    types::{ValidTemplateName, ValidVariantName},
};

pub struct Variant {
    pub name: ValidVariantName,
    pub path: PathBuf,
    pub template_name: ValidTemplateName,
    pub template_path: PathBuf,
}

impl Variant {
    pub fn new(template: &Template, name: &ValidVariantName) -> Self {
        let path = template.path.join(name.as_str());
        Self {
            name: name.clone(),
            path,
            template_name: template.name.clone(),
            template_path: template.path.clone(),
        }
    }
}
