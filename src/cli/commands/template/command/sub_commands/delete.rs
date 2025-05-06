use crate::cli::commands::template::{
    command::{utils::parse_template, TemplateInput},
    TemplateError,
};

#[derive(clap::Parser, Debug)]
#[command()]
pub struct DeleteSubcommand {
    #[arg(value_parser = parse_template, value_name = "Template Title")]
    pub template: Option<TemplateInput>,
}

impl DeleteSubcommand {
    pub fn execute(&self) -> Result<(), TemplateError> {
        Ok(())
    }
}
