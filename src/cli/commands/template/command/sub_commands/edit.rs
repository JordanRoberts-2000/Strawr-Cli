use crate::cli::commands::template::{
    command::{helpers::parse_template, manager::TemplateManager, utils::Template, TemplateInput},
    TemplateError,
};

#[derive(clap::Parser, Debug)]
#[command()]
pub struct EditSubcommand {
    #[arg(value_parser = parse_template, value_name = "Template Title")]
    pub template: Option<TemplateInput>,
}

impl EditSubcommand {
    pub fn execute(&self, manager: &TemplateManager) -> Result<(), TemplateError> {
        if let Some((template, variant)) = &self.template {
            let template = Template::new(&template, &manager.templates_path)?;
            manager.open_template(&template, &variant.as_deref())?;
            return Ok(());
        }

        let template = manager.select_template("Select Template:")?;
        if template.has_variants()? {
            let variant =
                manager.select_variant_or_default(&template, "Select variant to edit:")?;
            manager.open_template(&template, &Some(&variant))?;
        } else {
            manager.open_template(&template, &None)?;
        }

        Ok(())
    }
}
