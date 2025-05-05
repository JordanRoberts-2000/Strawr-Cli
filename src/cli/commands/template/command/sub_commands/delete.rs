use crate::cli::commands::template::{
    command::{helpers::parse_template, manager::TemplateManager, utils::Template, TemplateInput},
    TemplateError,
};

#[derive(clap::Parser, Debug)]
#[command()]
pub struct DeleteSubcommand {
    #[arg(value_parser = parse_template, value_name = "Template Title")]
    pub template: Option<TemplateInput>,

    #[arg(short, long, action = clap::ArgAction::SetTrue, conflicts_with = "template")]
    pub variant: bool,
}

impl DeleteSubcommand {
    pub fn execute(&self, manager: &TemplateManager) -> Result<(), TemplateError> {
        // if let Some((template, variant)) = &self.template {
        //     let template =
        //         Template::new(&template, &manager.templates_path)?.delete(&variant.as_deref())?;
        //     return Ok(());
        // }

        // if self.variant {
        //     let template = manager.select_template("Select a template to add a variant to:")?;
        //     let variant = manager.ctx.input.text("Variant title:")?;

        //     template.delete(&Some(&variant))?;
        //     return Ok(());
        // }

        // let input = manager.ctx.input.text("New Template title:")?;
        // let template = Template::new(&input, &manager.templates_path)?.delete(&None)?;

        Ok(())
    }
}
