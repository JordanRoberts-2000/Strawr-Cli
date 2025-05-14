use crate::template::{
    types::{ParsedTemplateInput, ValidVariantName},
    utils::template_parser,
};

#[derive(clap::Parser, Debug)]
#[command()]
pub struct RenameSubcommand {
    #[arg(value_parser = template_parser, value_name = "New Template Title")]
    pub template: Option<ParsedTemplateInput>,

    #[arg(short, long, num_args = 0..=1, requires = "template")]
    pub variant: Option<Option<ValidVariantName>>,
}
