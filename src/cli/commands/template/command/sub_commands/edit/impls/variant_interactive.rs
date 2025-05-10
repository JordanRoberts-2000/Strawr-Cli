use crate::cli::commands::template::{
    command::sub_commands::edit::{context::EditSubcommandContext, EditSubcommand},
    manager::TemplateManager,
    TemplateError,
};

impl EditSubcommand {
    pub fn edit_variant_interactive(
        &self,
        manager: &TemplateManager,
        ctx: &EditSubcommandContext,
    ) -> Result<(), TemplateError> {
        let template = manager.select_template("Select a template to see variants:")?;
        let variant = manager.select_variant(&template, "Select variant to edit:")?;

        ctx.service
            .launch_editor(&ctx.editor, &variant.path)
            .map_err(TemplateError::EditorLauncher)
    }
}
