use std::path::PathBuf;

use crate::{
    cli::commands::template::{command::sub_commands::TemplateSubcommands, TemplateError},
    utils::validation::{reserved, slug},
};

use super::Template;

impl Template {
    pub fn new(name: &str, templates_path: &PathBuf) -> Result<Self, TemplateError> {
        // already exists
        if templates_path.join(&name).is_dir() {
            return Ok(Self {
                name: name.to_string(),
                path: templates_path.join(&name),
            });
        }

        // doesnt exist, needs validating
        let valid_slug = slug(name).map_err(TemplateError::Validation)?;
        reserved::<TemplateSubcommands>(&valid_slug).map_err(TemplateError::Validation)?;

        if !templates_path.is_dir() {
            return Err(TemplateError::Validation(format!(
                "templates_path invalid '{}'",
                templates_path.display()
            )));
        }

        let path = templates_path.join(&valid_slug);

        Ok(Self {
            name: valid_slug,
            path,
        })
    }
}
