use std::path::PathBuf;

use crate::{
    template::{constants::TEMPLATES_FOLDER_NAME, TemplateService},
    traits::ToService,
    utils::{editor::EditorLauncher, input::CliInput},
    CliContext,
};

pub struct RenameSubcommandContext<'a> {
    pub templates_path: PathBuf,
    pub editor_launcher: &'a dyn EditorLauncher,
    pub input: &'a dyn CliInput,
}

impl<'a> RenameSubcommandContext<'a> {
    pub fn new(ctx: &'a CliContext) -> Self {
        let templates_path = ctx.storage_dir.join(TEMPLATES_FOLDER_NAME);
        let editor_launcher = ctx.service.editor_launcher.as_ref();
        let input = ctx.service.input.as_ref();

        Self {
            templates_path,
            editor_launcher,
            input,
        }
    }
}

impl<'a> ToService<'a, TemplateService<'a>> for RenameSubcommandContext<'a> {
    fn to_service(&'a self) -> TemplateService<'a> {
        TemplateService::new(self.input, self.editor_launcher, &self.templates_path)
    }
}
