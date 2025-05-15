use crate::{
    services::editor_launcher::Editor,
    template::{
        types::{ParsedTemplateInput, ValidVariantName},
        utils::template_parser,
    },
};

#[derive(clap::Parser, Debug)]
#[command()]
pub struct CreateSubcommand {
    #[arg(value_parser = template_parser, value_name = "Template[:Variant]")]
    pub template: Option<ParsedTemplateInput>,

    #[arg(short, long, num_args = 0..=1, requires = "template")]
    pub variant: Option<Option<ValidVariantName>>,

    #[arg(short, long, ignore_case = true)]
    pub editor: Option<Editor>,
}
