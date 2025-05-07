use crate::{
    cli::commands::template::{service::TemplateService, TemplateError},
    state::AppContext,
};

use super::{context::RenameSubcommandContext, RenameSubcommand};

impl RenameSubcommand {
    pub fn execute(&self, ctx: &AppContext) -> Result<(), TemplateError> {
        let ctx = RenameSubcommandContext::new(self, ctx);
        let service = TemplateService::from(&ctx);

        if let Some((raw_template, raw_variant)) = &self.template {
            return self.rename_from_input(&service, raw_template, raw_variant.as_deref());
        }

        if self.variant {
            return self.rename_variant_interactive(&service);
        }

        self.rename_template_interactive(&service)
    }
}
