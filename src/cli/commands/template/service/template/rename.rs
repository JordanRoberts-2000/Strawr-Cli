use std::fs::rename;

use crate::{
    cli::commands::template::{
        command::sub_commands::TemplateSubcommands, service::TemplateService, utils::Template,
        TemplateError,
    },
    error::IoError,
    utils::validation::{reserved, slug},
};

impl<'a> TemplateService<'a> {
    pub fn rename_template(
        &self,
        template: &Template,
        to: &str,
    ) -> Result<Template, TemplateError> {
        let valid_slug = slug(to).map_err(TemplateError::Validation)?;
        reserved::<TemplateSubcommands>(&valid_slug).map_err(TemplateError::Validation)?;

        let new_template_path = self.templates_path.join(&valid_slug);

        if new_template_path.exists() {
            return Err(TemplateError::Validation(format!(
                "Rename failed as '{}' is an existing template",
                template.name
            )));
        }

        rename(&template.path, &new_template_path)
            .map_err(|e| IoError::Rename(e, template.path.clone(), new_template_path.clone()))?;

        Ok(Template::new(&valid_slug, &new_template_path))
    }
}
