use crate::template::{TemplateCommand, TemplateContext, TemplateError, TemplateManager};

impl TemplateCommand {
    pub fn handle_creating_initial_template(
        &self,
        ctx: &TemplateContext,
        manager: &TemplateManager,
    ) -> Result<(), TemplateError> {
        let msg = "No templates currently exist, would you like to create one?";

        if ctx.service.prompt.confirm(msg)? {
            let input = ctx.service.prompt.text("Enter template name:")?;
            let template = manager.create_template(&input)?;

            ctx.service
                .launch_editor(&ctx.editor, &template.default_variant_path)
                .map_err(TemplateError::EditorLauncher)?;
        }

        Ok(())
    }
}
