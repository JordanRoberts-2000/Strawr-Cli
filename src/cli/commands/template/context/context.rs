use std::path::PathBuf;

use crate::{
    services::editor_launcher::Editor,
    template::{constants::TEMPLATES_FOLDER_NAME, types::TemplateInput, TemplateCommand},
    CliContext,
};

pub struct TemplateContext<'a> {
    pub templates_path: PathBuf,
    pub template: &'a Option<TemplateInput>,
    pub variant: &'a Option<Option<String>>,
    pub output: &'a PathBuf,
    pub editor: &'a Editor,
    pub backend: &'a Option<TemplateInput>,
    pub frontend: &'a Option<TemplateInput>,
    pub backend_folder_title: &'a String,
    pub frontend_folder_title: &'a String,
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
            backend: &args.backend,
            frontend: &args.frontend,
            backend_folder_title: &ctx.config.template.backend_folder_title,
            frontend_folder_title: &ctx.config.template.frontend_folder_title,
        }
    }
}
