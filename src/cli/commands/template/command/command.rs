use super::sub_commands::*;

use {std::path::PathBuf, strum_macros::VariantNames};

use crate::{
    services::editor_launcher::Editor,
    template::{
        types::{ParsedTemplateInput, ValidVariantName},
        utils::template_parser,
        TemplateController, TemplateError,
    },
    validation::adaptors::clap::validate,
    CliContext,
};

#[derive(clap::Parser, Debug)]
pub struct TemplateCommand {
    #[command(subcommand)]
    pub subcommand: Option<TemplateSubcommand>,

    #[arg(value_parser = template_parser, value_name = "Template[:Variant]")]
    pub template: Option<ParsedTemplateInput>,

    #[arg(short, long, num_args = 0..=1, requires = "template")]
    pub variant: Option<Option<ValidVariantName>>,

    #[arg(long, short, value_parser = template_parser, conflicts_with_all = ["variant", "template"])]
    pub backend: Option<ParsedTemplateInput>,

    #[arg(long, short, value_parser = template_parser, conflicts_with_all = ["variant", "template"])]
    pub frontend: Option<ParsedTemplateInput>,

    #[arg(long, short, default_value = ".", value_parser = validate::existing_dir)]
    pub output: PathBuf,

    #[arg(short, long, ignore_case = true)]
    pub editor: Option<Editor>,
}

#[derive(clap::Subcommand, Debug, VariantNames)]
#[strum(serialize_all = "lowercase")]
pub enum TemplateSubcommand {
    Create(CreateSubcommand),
    Rename(RenameSubcommand),
    Edit(EditSubcommand),
    Delete(DeleteSubcommand),
}

impl TemplateCommand {
    pub fn execute(&self, ctx: &CliContext) -> Result<(), TemplateError> {
        let controller = TemplateController::new(&ctx);
        controller.handle_command(self, &ctx)
    }
}
