use std::path::PathBuf;

use crate::{
    template::{constants::TEMPLATES_FOLDER_NAME, TemplateManager},
    traits::ToManager,
    CliContext, CliService,
};

pub struct DeleteSubcommandContext<'a> {
    pub templates_path: PathBuf,
    pub service: &'a CliService,
}

impl<'a> DeleteSubcommandContext<'a> {
    pub fn new(ctx: &'a CliContext) -> Self {
        let templates_path = ctx.storage_dir.join(TEMPLATES_FOLDER_NAME);

        Self {
            templates_path,
            service: &ctx.service,
        }
    }
}

impl<'a> ToManager<'a, TemplateManager<'a>> for DeleteSubcommandContext<'a> {
    fn to_manager(&'a self) -> TemplateManager<'a> {
        TemplateManager::new(&self.service, &self.templates_path)
    }
}
