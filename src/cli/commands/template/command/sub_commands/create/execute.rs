use crate::{
    cli::commands::template::{service::TemplateService, TemplateError},
    state::AppContext,
};

use super::{context::CreateSubcommandContext, CreateSubcommand};

impl CreateSubcommand {
    pub fn execute(&self, ctx: &AppContext) -> Result<(), TemplateError> {
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
