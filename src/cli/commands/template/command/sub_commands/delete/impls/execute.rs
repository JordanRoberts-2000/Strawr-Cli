use crate::{
    template::{
        sub_commands::delete::{DeleteSubcommand, DeleteSubcommandContext},
        TemplateError, TemplateManager,
    },
    CliContext,
};

impl DeleteSubcommand {
    pub fn execute(&self, ctx: &CliContext) -> Result<(), TemplateError> {
        let ctx = DeleteSubcommandContext::new(&ctx);
        let manager = TemplateManager::from(&ctx);

        if let Some((raw_template, raw_variant)) = &self.template {
            return self.delete_from_input(&manager, raw_template, raw_variant.as_deref());
        }

        if self.variant {
            return self.delete_variant_interactive(&manager);
        }

        self.delete_template_interactive(&manager)
    }
}
