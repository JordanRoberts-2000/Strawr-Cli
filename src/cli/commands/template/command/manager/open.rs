use crate::cli::commands::template::{
    command::{execute::TemplateInput, manager::TemplateManager},
    TemplateError,
};

impl<'a, T: TemplateInput> TemplateManager<'a, T> {
    pub fn open_template(
        &self,
        template: &str,
        variant: &Option<String>,
    ) -> Result<(), TemplateError> {
        let mut path = self.templates_path.join(template);

        match variant {
            Some(v) => path = path.join(v),
            None => path = path.join("default"),
        };

        self.ctx.config.default_editor.open(path)?;

        Ok(())
    }
}
