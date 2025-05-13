use clap::{value_parser, Parser};

use crate::{
    services::editor_launcher::Editor,
    template::{
        types::{TemplateInput, VariantInput},
        utils::parse_template,
    },
};

#[derive(Parser, Debug)]
#[command()]
pub struct CreateSubcommand {
    #[arg(value_parser = parse_template, value_name = "New Template Title")]
    pub template: Option<TemplateInput>,

    #[arg(short, long, num_args = 0..=1, requires = "template", value_parser = value_parser!(String))]
    pub variant: VariantInput,

    #[arg(short, long, ignore_case = true)]
    pub editor: Option<Editor>,
}
