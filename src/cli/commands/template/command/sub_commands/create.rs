use crate::{
    cli::commands::template::{command::manager::TemplateManager, TemplateError},
    utils::validation::{reserved, slug},
};

use super::TemplateSubcommands;

fn validate_template(s: &str) -> Result<String, String> {
    let valid_slug = slug(s)?;
    reserved::<TemplateSubcommands>(&valid_slug.as_str())?;

    Ok(valid_slug)
}

#[derive(clap::Parser, Debug)]
#[command()]
pub struct CreateSubcommand {
    #[arg(value_name = "New Template Title", value_parser = validate_template)]
    pub template: Option<String>,

    #[arg(short, long, action = clap::ArgAction::SetTrue, conflicts_with = "template")]
    pub variant: bool,
}

impl CreateSubcommand {
    pub fn execute(&self, manager: &TemplateManager) -> Result<(), TemplateError> {
        if let Some(template) = &self.template {
            return manager.create_template(&template, None);
        }

        if self.variant {
            return Self::handle_variant(&manager);
        }

        Self::handle_no_input(&manager)
    }

    fn handle_variant(manager: &TemplateManager) -> Result<(), TemplateError> {
        let template = manager.select_template("Select a template to add a variant to:")?;
        let variant = manager.ctx.input.text("Variant title:")?;

        let valid_slug = slug(&variant).map_err(TemplateError::Validation)?;

        if valid_slug == "default" {
            return Err(TemplateError::Validation(
                "'default' is a reserved value".to_string(),
            ));
        }

        manager.create_template(&template, Some(&valid_slug))?;

        Ok(())
    }

    fn handle_no_input(manager: &TemplateManager) -> Result<(), TemplateError> {
        let input = manager.ctx.input.text("New Template title:")?;
        let template = validate_template(&input).map_err(TemplateError::Validation)?;

        manager.create_template(&template, None)
    }
}
