use crate::{
    cli::commands::template::{
        command::{service::TemplateService, utils::Template},
        TemplateError,
    },
    utils::Editor,
};

impl<'a> TemplateService<'a> {
    pub fn open_template(&self, template: &Template, editor: &Editor) -> Result<(), TemplateError> {
        self.editor_launcher.open(editor, &template.path)?;

        Ok(())
    }
}
