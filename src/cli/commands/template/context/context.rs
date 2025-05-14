use std::path::PathBuf;

use crate::{
    services::editor_launcher::Editor,
    template::{
        types::{ParsedTemplateInput, ValidVariantName},
        TemplateCommand,
    },
    CliContext,
};

pub struct TemplateContext<'a> {
    pub template: &'a Option<ParsedTemplateInput>,
    pub variant: &'a Option<Option<ValidVariantName>>,
    pub output: &'a PathBuf,
    pub editor: &'a Editor,
    pub backend: &'a Option<ParsedTemplateInput>,
    pub frontend: &'a Option<ParsedTemplateInput>,
    pub backend_folder_title: &'a String,
    pub frontend_folder_title: &'a String,
}

impl<'a> TemplateContext<'a> {
    pub fn new(args: &'a TemplateCommand, ctx: &'a CliContext) -> Self {
        let editor = args.editor.as_ref().unwrap_or(&ctx.config.default_editor);

        Self {
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
