use crate::cli::commands::template::{command::manager::TemplateManager, TemplateError};

impl<'a> TemplateManager<'a> {
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

        self.ctx.editor.open(self.editor, &path)?;

        Ok(())
    }
}
