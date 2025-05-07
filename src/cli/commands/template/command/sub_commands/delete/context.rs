use std::path::PathBuf;

use crate::{
    cli::commands::template::{service::TemplateService, TEMPLATES_FOLDER_NAME},
    state::AppContext,
    traits::ToService,
    utils::{editor::EditorLauncher, input::CliInput, Editor},
};

use super::DeleteSubcommand;

pub struct DeleteSubcommandContext<'a> {
    pub templates_path: PathBuf,
    pub editor: &'a Editor,
    pub editor_launcher: &'a dyn EditorLauncher,
    pub input: &'a dyn CliInput,
}

impl<'a> DeleteSubcommandContext<'a> {
    pub fn new(args: &'a DeleteSubcommand, ctx: &'a AppContext) -> Self {
        let templates_path = ctx.storage_dir.join(TEMPLATES_FOLDER_NAME);
        let editor = args.editor.as_ref().unwrap_or(&ctx.config.default_editor);
        let editor_launcher = ctx.editor.as_ref();
        let input = ctx.input.as_ref();

        Self {
            templates_path,
            editor,
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
