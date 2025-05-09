use crate::{
    template::{
        sub_commands::create::{CreateSubcommand, CreateSubcommandContext},
        TemplateError, TemplateService,
    },
    CliContext,
};

impl CreateSubcommand {
    pub fn execute(&self, ctx: &CliContext) -> Result<(), TemplateError> {
        let ctx = CreateSubcommandContext::new(self, ctx);
        let service = TemplateService::from(&ctx);

        if let Some((raw_template, raw_variant)) = &self.template {
            return self.create_from_input(&service, &ctx, raw_template, raw_variant.as_deref());
        }

        if self.variant {
            return self.create_variant_interactive(&service, &ctx);
        }

        self.create_template_interactive(&service, &ctx)
    }
}
