use std::fs;

use crate::{
    cli::commands::template::{
        command::sub_commands::TemplateSubcommands, service::TemplateService, utils::Template,
        TemplateError, DEFAULT_FOLDER,
    },
    error::IoError,
    utils::validation::{reserved, slug},
};

impl<'a> TemplateService<'a> {
    pub fn create_template(&self, str: &str) -> Result<Template, TemplateError> {
        let template = slug(str).map_err(TemplateError::Validation)?;
        reserved::<TemplateSubcommands>(&template).map_err(TemplateError::Validation)?;

        let template_path = self.templates_path.join(&template);

        if template_path.exists() {
            return Err(TemplateError::Validation(format!(
                "Template '{}' already exists",
                template
            )));
        }

        let default_path = template_path.join(DEFAULT_FOLDER);

        fs::create_dir_all(&default_path)
            .map_err(|e| IoError::CreateDir(e, default_path.clone()))?;

        Ok(Template::new(&template, &template_path))
    }
}
