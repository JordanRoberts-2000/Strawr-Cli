use std::path::PathBuf;

use crate::{
    template::{
        constants::TEMPLATES_FOLDER_NAME, types::TemplateInput, TemplateCommand, TemplateService,
    },
    traits::ToService,
    utils::{editor::EditorLauncher, input::CliInput, Editor},
    CliContext,
};

pub struct TemplateContext<'a> {
    pub templates_path: PathBuf,
    pub template: &'a Option<TemplateInput>,
    pub variant: &'a Option<Option<String>>,
    pub output: &'a PathBuf,
    pub editor: &'a Editor,
    pub editor_launcher: &'a dyn EditorLauncher,
    pub input: &'a dyn CliInput,
}

impl<'a> TemplateContext<'a> {
    pub fn new(args: &'a TemplateCommand, ctx: &'a CliContext) -> Self {
        let templates_path = ctx.storage_dir.join(TEMPLATES_FOLDER_NAME);
        let editor = args.editor.as_ref().unwrap_or(&ctx.config.default_editor);
        let editor_launcher = ctx.service.editor_launcher.as_ref();
        let input = ctx.service.prompt.as_ref();
        let template = &args.template;
        let variant = &args.variant;
        let output = &args.output;

        Self {
            templates_path,
            editor,
            editor_launcher,
            input,
            output,
            template,
            variant,
        }
    }
}

impl<'a> ToService<'a, TemplateService<'a>> for TemplateContext<'a> {
    fn to_service(&'a self) -> TemplateService<'a> {
        TemplateService::new(self.input, self.editor_launcher, &self.templates_path)
    }
}
