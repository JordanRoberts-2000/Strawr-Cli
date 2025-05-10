use std::path::PathBuf;

use crate::{
    services::editor_launcher::Editor,
    template::{constants::TEMPLATES_FOLDER_NAME, TemplateManager},
    traits::ToManager,
    CliContext, CliService,
};

use super::command::CreateSubcommand;

pub struct CreateSubcommandContext<'a> {
    pub templates_path: PathBuf,
    pub editor: &'a Editor,
    pub service: &'a CliService,
}

impl<'a> CreateSubcommandContext<'a> {
    pub fn new(args: &'a CreateSubcommand, ctx: &'a CliContext) -> Self {
        let templates_path = ctx.storage_dir.join(TEMPLATES_FOLDER_NAME);
        let editor = args.editor.as_ref().unwrap_or(&ctx.config.default_editor);

        Self {
            templates_path,
            editor,
            service: &ctx.service,
        }
    }
}

impl<'a> ToManager<'a, TemplateManager<'a>> for CreateSubcommandContext<'a> {
    fn to_manager(&'a self) -> TemplateManager<'a> {
        TemplateManager::new(&self.service, &self.templates_path)
    }
}
