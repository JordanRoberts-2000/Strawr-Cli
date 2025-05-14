use crate::{
    services::editor_launcher::Editor,
    template::{
        types::{ValidTemplateName, ValidVariantName},
        utils::template_parser,
    },
};

#[derive(clap::Parser, Debug)]
#[command()]
pub struct CreateSubcommand {
    #[arg(value_parser = template_parser, value_name = "New Template Title")]
    pub template: Option<ValidTemplateName>,

    #[arg(short, long, num_args = 0..=1, requires = "template")]
    pub variant: Option<Option<ValidVariantName>>,

    #[arg(short, long, ignore_case = true)]
    pub editor: Option<Editor>,
}
