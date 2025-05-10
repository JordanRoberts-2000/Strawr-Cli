use crate::cli::commands::template::{
    command::sub_commands::edit::{context::EditSubcommandContext, EditSubcommand},
    manager::TemplateManager,
    TemplateError,
};

impl EditSubcommand {
    pub fn edit_template_interactive(
        &self,
        manager: &TemplateManager,
        ctx: &EditSubcommandContext,
    ) -> Result<(), TemplateError> {
        let template = manager.select_template("Template to edit:")?;

        ctx.service
            .launch_editor(&ctx.editor, &template.default_variant_path)
            .map_err(TemplateError::EditorLauncher)
    }
}
