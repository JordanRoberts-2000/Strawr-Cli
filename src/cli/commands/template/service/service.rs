use std::path::PathBuf;

use crate::{
    services::{
        cli::traits::HasEditorLauncherService,
        fs::{CliFsRepository, FsRepository},
    },
    template::constants::TEMPLATES_FOLDER_NAME,
    CliContext, CliService,
};

pub struct TemplateService<'ctx> {
    pub templates_path: PathBuf,
    pub cli_svc: &'ctx CliService,
    pub fs: Box<dyn FsRepository>,
}

impl<'ctx> TemplateService<'ctx> {
    pub fn new(ctx: &'ctx CliContext) -> Self {
        let templates_path = ctx.storage_dir.join(TEMPLATES_FOLDER_NAME);
        Self {
            templates_path,
            cli_svc: &ctx.service,
            fs: Box::new(CliFsRepository),
        }
    }
}

impl<'ctx> HasEditorLauncherService for TemplateService<'ctx> {
    fn cli(&self) -> &CliService {
        self.cli_svc
    }
}
