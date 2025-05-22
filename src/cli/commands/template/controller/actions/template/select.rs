use crate::template::{
    models::{markers::Exists, Template},
    TemplateController, TemplateError,
};

impl<'c> TemplateController<'c> {
    pub fn select_template(&self) -> Result<Template<Exists>, TemplateError> {
        let templates = self.service.get_templates()?;
        let input = self.view.select_template(&templates)?;

        Ok(Template::new(&input, &self.service.templates_path))
    }
}
