use crate::cli::commands::template::{command::manager::TemplateManager, TemplateError};

impl TemplateManager {
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

        self.editor.open(path)?;

        Ok(())
    }
}
