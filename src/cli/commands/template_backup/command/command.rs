use std::path::PathBuf;

use crate::{
    services::editor_launcher::Editor,
    template::{types::TemplateInput, utils::parse_template, TemplateSubcommand},
    utils::validation::existing_dir,
};

#[derive(clap::Parser, Debug)]
pub struct TemplateCommand {
    #[command(subcommand)]
    pub subcommand: Option<TemplateSubcommand>,

    #[arg(value_parser = parse_template)]
    pub template: Option<TemplateInput>,

    #[arg(short, long, num_args = 0..=1, requires = "template")]
    pub variant: Option<Option<String>>,

    #[arg(long, short, value_parser = parse_template, conflicts_with_all = ["variant", "template"])]
    pub backend: Option<TemplateInput>,

    #[arg(long, short, value_parser = parse_template, conflicts_with_all = ["variant", "template"])]
    pub frontend: Option<TemplateInput>,

    #[arg(long, short, default_value = ".", value_parser = existing_dir)]
    pub output: PathBuf,

    #[arg(short, long, ignore_case = true)]
    pub editor: Option<Editor>,
}
