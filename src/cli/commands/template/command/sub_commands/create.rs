use crate::cli::commands::template::{
    command::{utils::parse_template, TemplateInput},
    TemplateError,
};

#[derive(clap::Parser, Debug)]
#[command()]
pub struct CreateSubcommand {
    #[arg(value_parser = parse_template, value_name = "New Template Title")]
    pub template: Option<TemplateInput>,

    #[arg(short, long, action = clap::ArgAction::SetTrue, conflicts_with = "template")]
    pub variant: bool,
}

impl CreateSubcommand {
    pub fn execute(&self) -> Result<(), TemplateError> {
        // if let Some((template, variant)) = &self.template {
        //     let template =
        //         Template::new(&template, &manager.templates_path)?.create(&variant.as_deref())?;
        //     manager.open_template(&template, &variant.as_deref())?;
        //     return Ok(());
        // }

        // if self.variant {
        //     let template = manager.select_template("Select a template to add a variant to:")?;
        //     let variant = manager.ctx.input.text("Variant title:")?;

        //     template.create(&Some(&variant))?;
        //     manager.open_template(&template, &Some(&variant))?;
        //     return Ok(());
        // }

        // let input = manager.ctx.input.text("New Template title:")?;
        // let template = Template::new(&input, &manager.templates_path)?.create(&None)?;
        // manager.open_template(&template, &None)?;

        Ok(())
    }
}
