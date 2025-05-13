use super::sub_commands::*;

use {
    clap::{value_parser, Parser},
    std::path::PathBuf,
    strum_macros::VariantNames,
};

use crate::{
    services::editor_launcher::Editor,
    template::{
        types::{TemplateInput, VariantInput},
        utils::parse_template,
        TemplateController, TemplateError,
    },
    utils::validation::existing_dir,
    CliContext,
};

#[derive(Parser, Debug)]
pub struct TemplateCommand {
    #[command(subcommand)]
    pub subcommand: Option<TemplateSubcommand>,

    #[arg(value_parser = parse_template)]
    pub template: Option<TemplateInput>,

    #[arg(short, long, num_args = 0..=1, requires = "template", value_parser = value_parser!(String))]
    pub variant: VariantInput,

    #[arg(long, short, value_parser = parse_template, conflicts_with_all = ["variant", "template"])]
    pub backend: Option<TemplateInput>,

    #[arg(long, short, value_parser = parse_template, conflicts_with_all = ["variant", "template"])]
    pub frontend: Option<TemplateInput>,

    #[arg(long, short, default_value = ".", value_parser = existing_dir)]
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
        controller.execute(self, &ctx)
    }
}
