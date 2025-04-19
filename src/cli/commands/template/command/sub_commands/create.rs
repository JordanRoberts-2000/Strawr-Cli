use crate::cli::commands::template::{
    command::{helpers::parse_input, manager::TemplateManager},
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
    pub fn execute(&self, manager: &TemplateManager) -> Result<(), TemplateError> {
        let (template, variant) = parse_input(&self.template, &self.variant)?;
        log::trace!("Input parsed - template: '{template}', variant: '{variant:?}'");

        if variant.is_some() && !manager.templates.contains(&template) {
            return Err(TemplateError::CreatingVariantWithoutDefault);
        }

        manager.create_template(&template, &variant)?;

        Ok(())
    }
}
