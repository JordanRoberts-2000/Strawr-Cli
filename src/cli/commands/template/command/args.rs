use crate::utils::Editor;

use super::sub_commands::TemplateSubcommands;

#[derive(clap::Parser, Debug)]
pub struct TemplateCommand {
    #[arg()]
    pub template: Option<String>,

    #[command(subcommand)]
    pub subcommand: Option<TemplateSubcommands>,

    #[arg(long, short)]
    pub variant: Option<String>,

    #[arg(long, short)]
    pub backend: Option<String>,

    #[arg(long, short)]
    pub frontend: Option<String>,

    #[arg(short, long, value_enum, ignore_case = true)]
    pub editor: Option<Editor>,
}
