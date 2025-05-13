use clap::{value_parser, Parser};

use crate::template::{
    types::{TemplateInput, VariantInput},
    utils::parse_template,
};

#[derive(Parser, Debug)]
#[command()]
pub struct DeleteSubcommand {
    #[arg(value_parser = parse_template, value_name = "New Template Title")]
    pub template: Option<TemplateInput>,

    #[arg(short, long, num_args = 0..=1, requires = "template", value_parser = value_parser!(String))]
    pub variant: VariantInput,
}
