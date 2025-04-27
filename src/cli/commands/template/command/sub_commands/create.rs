use crate::cli::commands::template::{
    command::{execute::TemplateInput, helpers::parse_input, manager::TemplateManager},
    TemplateError,
};

#[derive(clap::Parser, Debug)]
#[command()]
pub struct CreateSubcommand {
    #[arg()]
    pub template: String,

    #[arg(long, short)]
    pub variant: Option<String>,
}

impl CreateSubcommand {
    pub fn execute<'a, T: TemplateInput>(
        &self,
        manager: &TemplateManager<'a, T>,
    ) -> Result<(), TemplateError> {
        let (template, variant) = parse_input(&self.template, &self.variant)?;
        log::trace!("Input parsed - template: '{template}', variant: '{variant:?}'");

        manager.create_template(&template, variant.as_deref())?;

        Ok(())
    }
}
