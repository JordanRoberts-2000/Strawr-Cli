use crate::{
    error::IoError,
    template::{utils::Template, TemplateError, TemplateManager},
    utils::fs::subfolders,
};

impl<'a> TemplateManager<'a> {
    pub fn select_template(&self, msg: &str) -> Result<Template, TemplateError> {
        let templates = subfolders(&self.templates_path)
            .map_err(|e| IoError::ReadDir(e, self.templates_path.to_path_buf()))?;
        let input = self.service.prompt().search(&templates, msg)?;

        let template_path = self.templates_path.join(&input);
        Ok(Template::new(&input, &template_path))
    }
}
