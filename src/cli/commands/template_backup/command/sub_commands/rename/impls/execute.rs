use crate::{
    template::{
        sub_commands::rename::{RenameSubcommand, RenameSubcommandContext},
        TemplateError, TemplateManager,
    },
    CliContext,
};

impl RenameSubcommand {
    pub fn execute(&self, ctx: &CliContext) -> Result<(), TemplateError> {
        let ctx = RenameSubcommandContext::new(ctx);
        let manager = TemplateManager::from(&ctx);

        if let Some((raw_template, raw_variant)) = &self.template {
            return self.rename_from_input(&manager, &ctx, raw_template, raw_variant.as_deref());
        }

        if self.variant {
            return self.rename_variant_interactive(&manager, &ctx);
        }

        self.rename_template_interactive(&manager, &ctx)
    }
}
