use crate::cli::commands::template::{
    command::sub_commands::edit::{context::EditSubcommandContext, EditSubcommand},
    service::TemplateService,
    TemplateError,
};

impl EditSubcommand {
    pub fn edit_variant_interactive(
        &self,
        service: &TemplateService,
        ctx: &EditSubcommandContext,
    ) -> Result<(), TemplateError> {
        let template = service.select_template("Select a template to see variants:")?;
        let variant = service.select_variant(&template, "Select variant to edit:")?;
        service.launch_editor(&ctx.editor, &variant.path)
    }
}
