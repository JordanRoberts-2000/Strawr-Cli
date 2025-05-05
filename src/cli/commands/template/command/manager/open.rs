use std::path::PathBuf;

use crate::cli::commands::template::{
    command::{manager::TemplateManager, utils::Template},
    TemplateError, DEFAULT_FOLDER,
};

impl<'a> TemplateManager<'a> {
    pub fn open_template(
        &self,
        template: &Template,
        variant: &Option<&str>,
    ) -> Result<(), TemplateError> {
        let mut path = template.path.clone();

        match variant {
            Some(v) => {
                path = path.join(v);
                if !path.exists() {
                    return Err(TemplateError::VariantNotFound {
                        template: template.name.clone(),
                        variant: v.to_string(),
                    });
                }
            }
            None => {
                path = path.join(DEFAULT_FOLDER);
                if !path.exists() {
                    return Err(TemplateError::TemplateNotFound(template.name.clone()));
                }
            }
        };

        self.ctx.editor.open(self.editor, &path)?;

        Ok(())
    }
}
