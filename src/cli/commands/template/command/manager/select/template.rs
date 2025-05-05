use crate::{
    cli::commands::template::{
        command::{manager::TemplateManager, utils::Template},
        TemplateError,
    },
    error::IoError,
    utils::fs::subfolders,
};

impl<'a> TemplateManager<'a> {
    pub fn select_template(&self, msg: &str) -> Result<Template, TemplateError> {
        let templates = subfolders(&self.templates_path)
            .map_err(|e| IoError::ReadDir(e, self.templates_path.clone()))?;

        if templates.is_empty() {
            return Err(TemplateError::NoTemplatesExist);
        }

        let selected_template: String = self.ctx.input.select(&templates, msg)?;

        let valid_template = Template::new(&selected_template, &self.templates_path)?;
        log::debug!("User selected template: '{}'", valid_template.name);

        Ok(valid_template)
    }
}
