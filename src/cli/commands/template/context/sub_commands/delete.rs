use std::path::PathBuf;

use crate::{template::constants::TEMPLATES_FOLDER_NAME, CliContext, CliService};

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
