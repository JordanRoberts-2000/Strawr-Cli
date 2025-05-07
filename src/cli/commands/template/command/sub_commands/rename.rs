use crate::cli::commands::template::{
    command::TemplateInput, utils::parse_template, TemplateError,
};

#[derive(clap::Parser, Debug)]
#[command()]
pub struct RenameSubcommand {
    #[arg(value_parser = parse_template, value_name = "Template Title")]
    pub template: Option<TemplateInput>,
}

impl RenameSubcommand {
    pub fn execute(&self) -> Result<(), TemplateError> {
        Ok(())
    }
}
