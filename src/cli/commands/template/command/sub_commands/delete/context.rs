use std::path::PathBuf;

use crate::{
    template::{constants::TEMPLATES_FOLDER_NAME, TemplateService},
    traits::ToService,
    utils::{editor::EditorLauncher, input::CliInput, Editor},
    CliContext,
};

use super::DeleteSubcommand;

pub struct DeleteSubcommandContext<'a> {
    pub templates_path: PathBuf,
    pub editor_launcher: &'a dyn EditorLauncher,
    pub input: &'a dyn CliInput,
}

impl<'a> DeleteSubcommandContext<'a> {
    pub fn new(args: &'a DeleteSubcommand, ctx: &'a CliContext) -> Self {
        let templates_path = ctx.storage_dir.join(TEMPLATES_FOLDER_NAME);
        let editor_launcher = ctx.service.editor_launcher.as_ref();
        let input = ctx.service.prompt.as_ref();

        Self {
            templates_path,
            editor_launcher,
            input,
        }
    }
}

impl<'a> ToService<'a, TemplateService<'a>> for DeleteSubcommandContext<'a> {
    fn to_service(&'a self) -> TemplateService<'a> {
        TemplateService::new(self.input, self.editor_launcher, &self.templates_path)
    }
}
