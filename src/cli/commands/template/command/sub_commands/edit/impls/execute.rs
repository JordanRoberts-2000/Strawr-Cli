use crate::{
    template::{
        sub_commands::edit::{EditSubcommand, EditSubcommandContext},
        TemplateError, TemplateService,
    },
    CliContext,
};

impl EditSubcommand {
    pub fn execute(&self, ctx: &CliContext) -> Result<(), TemplateError> {
        let ctx = EditSubcommandContext::new(self, ctx);
        let service = TemplateService::from(&ctx);

        if let Some((raw_template, raw_variant)) = &self.template {
            return self.edit_from_input(&service, &ctx, raw_template, raw_variant.as_deref());
        }

        if self.variant {
            return self.edit_variant_interactive(&service, &ctx);
        }

        self.edit_template_interactive(&service, &ctx)
    }
}
