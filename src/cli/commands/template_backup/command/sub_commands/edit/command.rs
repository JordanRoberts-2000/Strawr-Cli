use crate::{
    services::editor_launcher::Editor,
    template::{types::TemplateInput, utils::parse_template},
};

#[derive(clap::Parser, Debug)]
#[command()]
pub struct EditSubcommand {
    #[arg(value_parser = parse_template, value_name = "New Template Title")]
    pub template: Option<TemplateInput>,

    #[arg(short, long, action = clap::ArgAction::SetTrue, conflicts_with = "template")]
    pub variant: bool,

    #[arg(short, long, ignore_case = true)]
    pub editor: Option<Editor>,
}
