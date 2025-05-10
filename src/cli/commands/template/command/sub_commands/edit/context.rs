use std::path::PathBuf;

use crate::{
    template::{constants::TEMPLATES_FOLDER_NAME, TemplateManager},
    traits::ToManager,
    utils::Editor,
    CliContext, CliService,
};

use super::EditSubcommand;

pub struct EditSubcommandContext<'a> {
    pub templates_path: PathBuf,
    pub editor: &'a Editor,
    pub service: &'a CliService,
}

impl<'a> EditSubcommandContext<'a> {
    pub fn new(args: &'a EditSubcommand, ctx: &'a CliContext) -> Self {
        let templates_path = ctx.storage_dir.join(TEMPLATES_FOLDER_NAME);
        let editor = args.editor.as_ref().unwrap_or(&ctx.config.default_editor);

        Self {
            templates_path,
            editor,
            service: &ctx.service,
        }
    }
}

impl<'a> ToManager<'a, TemplateManager<'a>> for EditSubcommandContext<'a> {
    fn to_manager(&'a self) -> TemplateManager<'a> {
        TemplateManager::new(&self.service, &self.templates_path)
    }
}
