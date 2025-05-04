use crate::cli::commands::template::{
    command::{helpers::parse_template, manager::TemplateManager, TemplateInput},
    TemplateError,
};

#[derive(clap::Parser, Debug)]
#[command()]
pub struct CreateSubcommand {
    #[arg(value_parser = parse_template)]
    pub template: Option<TemplateInput>,

    #[arg(short, long, num_args = 0..=1, requires = "template")]
    pub variant: Option<Option<String>>,
}

impl CreateSubcommand {
    pub fn execute(&self, manager: &TemplateManager) -> Result<(), TemplateError> {
        // todo fix args
        // create -> input.text
        // create -v -> input.select then input.text
        Ok(())
    }
}
