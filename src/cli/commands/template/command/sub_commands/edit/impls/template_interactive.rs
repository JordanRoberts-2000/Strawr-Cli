use crate::cli::commands::template::{
    command::sub_commands::edit::{context::EditSubcommandContext, EditSubcommand},
    service::TemplateService,
    TemplateError,
};

impl EditSubcommand {
    pub fn edit_template_interactive(
        &self,
        service: &TemplateService,
        ctx: &EditSubcommandContext,
    ) -> Result<(), TemplateError> {
        let template = service.select_template("Template to edit:")?;
        service.launch_editor(&ctx.editor, &template.default_variant_path)
    }
}
