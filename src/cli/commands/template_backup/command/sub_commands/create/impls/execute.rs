use crate::{
    template::{
        sub_commands::create::{CreateSubcommand, CreateSubcommandContext},
        TemplateError, TemplateManager,
    },
    CliContext,
};

impl CreateSubcommand {
    pub fn execute(&self, ctx: &CliContext) -> Result<(), TemplateError> {
        let ctx = CreateSubcommandContext::new(self, ctx);
        let manager = TemplateManager::from(&ctx);

        if let Some((raw_template, raw_variant)) = &self.template {
            return self.create_from_input(&manager, &ctx, raw_template, raw_variant.as_deref());
        }

        if self.variant {
            return self.create_variant_interactive(&manager, &ctx);
        }

        self.create_template_interactive(&manager, &ctx)
    }
}
