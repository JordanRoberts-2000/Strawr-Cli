use std::path::PathBuf;

use crate::{
    template::{
        constants::TEMPLATES_FOLDER_NAME, types::TemplateInput, TemplateCommand, TemplateManager,
    },
    traits::ToManager,
    utils::Editor,
    CliContext, CliService,
};

pub struct TemplateContext<'a> {
    pub templates_path: PathBuf,
    pub template: &'a Option<TemplateInput>,
    pub variant: &'a Option<Option<String>>,
    pub output: &'a PathBuf,
    pub editor: &'a Editor,
    pub service: &'a CliService,
}

impl<'a> TemplateContext<'a> {
    pub fn new(args: &'a TemplateCommand, ctx: &'a CliContext) -> Self {
        let templates_path = ctx.storage_dir.join(TEMPLATES_FOLDER_NAME);
        let editor = args.editor.as_ref().unwrap_or(&ctx.config.default_editor);

        Self {
            templates_path,
            editor,
            output: &args.output,
            template: &args.template,
            variant: &args.variant,
            service: &ctx.service,
        }
    }
}

impl<'a> ToManager<'a, TemplateManager<'a>> for TemplateContext<'a> {
    fn to_manager(&'a self) -> TemplateManager<'a> {
        TemplateManager::new(&self.service, &self.templates_path)
    }
}
